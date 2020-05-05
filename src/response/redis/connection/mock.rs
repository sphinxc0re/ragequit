pub(crate) use super::RedisConnErr;

use super::super::Error as ManagerErr;
use super::super::RedisCmd;
use crate::config::Redis;
use crate::request::Timeline;

use futures::{Async, Poll};
use lru::LruCache;

type Result<T> = std::result::Result<T, RedisConnErr>;

#[derive(Debug)]
pub(in super::super) struct RedisConn {
    pub(in super::super) namespace: Option<String>,
    // TODO: eventually, it might make sense to have Mastodon publish to timelines with
    //       the tag number instead of the tag name.  This would save us from dealing
    //       with a cache here and would be consistent with how lists/users are handled.
    pub(in super::super) tag_name_cache: LruCache<i64, String>,
    pub(in super::super) input: Vec<u8>,
}

impl RedisConn {
    pub(in super::super) fn new(redis_cfg: &Redis) -> Result<Self> {
        Ok(Self {
            tag_name_cache: LruCache::new(1000),
            namespace: redis_cfg.namespace.clone().0,
            input: vec![0; 4096 * 4],
        })
    }
    pub(in super::super) fn poll_redis(&mut self, start: usize) -> Poll<usize, ManagerErr> {
        const BLOCK: usize = 4096 * 2;
        if self.input.len() < start + BLOCK {
            self.input.resize(self.input.len() * 2, 0);
            log::info!("Resizing input buffer to {} KiB.", self.input.len() / 1024);
            // log::info!("Current buffer: {}", String::from_utf8_lossy(&self.input));
        }

        use Async::*;
        //        self.input[start..start + BLOCK] = &"foo".as_bytes();
        let mut n = 0;
        for i in 0..BLOCK {
            if let Some(byte) = TEST_INPUT.get(start + i) {
                self.input[start + 1] = *byte;
                n += 1;
            }
        }

        Ok(Ready(n))
        // match self.primary.read(&mut self.input[start..start + BLOCK]) {
        //     Ok(n) => Ok(Ready(n)),
        //     Err(e) if matches!(e.kind(), io::ErrorKind::WouldBlock) => Ok(NotReady),
        //     Err(e) => {
        //         Ready(log::error!("{}", e));
        //         Ok(Ready(0))
        //     }
        // }
    }

    pub(crate) fn send_cmd(&mut self, _cmd: RedisCmd, _timelines: &[Timeline]) -> Result<()> {
        Ok(())
    }
}

const TEST_INPUT: &[u8] = r##"*3
$7
message
$15
timeline:public
$3790
{"event":"update","payload":{"id":"102775370117886890","created_at":"2019-09-11T18:42:19.000Z","in_reply_to_id":null,"in_reply_to_account_id":null,"sensitive":false,"spoiler_text":"","visibility":"unlisted","language":"en","uri":"https://mastodon.host/users/federationbot/statuses/102775346916917099","url":"https://mastodon.host/@federationbot/102775346916917099","replies_count":0,"reblogs_count":0,"favourites_count":0,"favourited":false,"reblogged":false,"muted":false,"content":"<p>Trending tags:<br><a href=\"https://mastodon.host/tags/neverforget\" class=\"mention hashtag\" rel=\"nofollow noopener\" target=\"_blank\">#<span>neverforget</span></a><br><a href=\"https://mastodon.host/tags/4styles\" class=\"mention hashtag\" rel=\"nofollow noopener\" target=\"_blank\">#<span>4styles</span></a><br><a href=\"https://mastodon.host/tags/newpipe\" class=\"mention hashtag\" rel=\"nofollow noopener\" target=\"_blank\">#<span>newpipe</span></a><br><a href=\"https://mastodon.host/tags/uber\" class=\"mention hashtag\" rel=\"nofollow noopener\" target=\"_blank\">#<span>uber</span></a><br><a href=\"https://mastodon.host/tags/mercredifiction\" class=\"mention hashtag\" rel=\"nofollow noopener\" target=\"_blank\">#<span>mercredifiction</span></a></p>","reblog":null,"account":{"id":"78","username":"federationbot","acct":"federationbot@mastodon.host","display_name":"Federation Bot","locked":false,"bot":false,"created_at":"2019-09-10T15:04:25.559Z","note":"<p>Hello, I am mastodon.host official semi bot.</p><p>Follow me if you want to have some updates on the view of the fediverse from here ( I only post unlisted ). </p><p>I also randomly boost one of my followers toot every hour !</p><p>If you don't feel confortable with me following you, tell me: unfollow  and I'll do it :)</p><p>If you want me to follow you, just tell me follow ! </p><p>If you want automatic follow for new users on your instance and you are an instance admin, contact me !</p><p>Other commands are private :)</p>","url":"https://mastodon.host/@federationbot","avatar":"https://instance.codesections.com/system/accounts/avatars/000/000/078/original/d9e2be5398629cf8.jpeg?1568127863","avatar_static":"https://instance.codesections.com/system/accounts/avatars/000/000/078/original/d9e2be5398629cf8.jpeg?1568127863","header":"https://instance.codesections.com/headers/original/missing.png","header_static":"https://instance.codesections.com/headers/original/missing.png","followers_count":16636,"following_count":179532,"statuses_count":50554,"emojis":[],"fields":[{"name":"More stats","value":"<a href=\"https://mastodon.host/stats.html\" rel=\"nofollow noopener\" target=\"_blank\"><span class=\"invisible\">https://</span><span class=\"\">mastodon.host/stats.html</span><span class=\"invisible\"></span></a>","verified_at":null},{"name":"More infos","value":"<a href=\"https://mastodon.host/about/more\" rel=\"nofollow noopener\" target=\"_blank\"><span class=\"invisible\">https://</span><span class=\"\">mastodon.host/about/more</span><span class=\"invisible\"></span></a>","verified_at":null},{"name":"Owner/Friend","value":"<span class=\"h-card\"><a href=\"https://mastodon.host/@gled\" class=\"u-url mention\" rel=\"nofollow noopener\" target=\"_blank\">@<span>gled</span></a></span>","verified_at":null}]},"media_attachments":[],"mentions":[],"tags":[{"name":"4styles","url":"https://instance.codesections.com/tags/4styles"},{"name":"neverforget","url":"https://instance.codesections.com/tags/neverforget"},{"name":"mercredifiction","url":"https://instance.codesections.com/tags/mercredifiction"},{"name":"uber","url":"https://instance.codesections.com/tags/uber"},{"name":"newpipe","url":"https://instance.codesections.com/tags/newpipe"}],"emojis":[],"card":null,"poll":null},"queued_at":1568227693541}"##.as_bytes();