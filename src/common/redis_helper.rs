use redis::{cluster::ClusterClient, Client, Commands, Connection};
use std::boxed::Box;
pub(crate) trait RedisImplem{
    fn find_key(&mut self, the_key:&str)-> redis::RedisResult<String>;
    fn find_token(&mut self, thetoken: &str) -> redis::RedisResult<String>;
    fn put_key(&mut self, the_redis_key: &str, the_key: &str) -> redis::RedisResult<String> ;
    fn put_token(&mut self, the_redis_key: &str, the_token: &str) -> redis::RedisResult<String> ;
    fn delete_key(&mut self, the_redis_key: &str) -> redis::RedisResult<String>;
    fn delete_token(&mut self, the_redis_key: &str) -> redis::RedisResult<String>;    
}
pub(crate) struct RedisProcessor{
    pub(crate) redis_impl:Box<dyn RedisImplem>,
}
pub(crate) struct RedisMockProcessor;
pub(crate) struct RedisRealProcessor {
    redis_key: redis::Connection,
    redis_jwt: redis::Connection,
}
impl RedisImplem for RedisRealProcessor {

    //TODO: find token to redis
    fn find_token(&mut self, thetoken: &str) -> redis::RedisResult<String> {
        self.redis_jwt.get(thetoken)
    }
    fn find_key(&mut self, thetoken: &str) -> redis::RedisResult<String> {
        self.redis_key.get(thetoken)
    }
    //TODO: put token to redis
    fn put_token(&mut self, the_redis_key: &str, the_token: &str) -> redis::RedisResult<String> {
        self.redis_jwt.set(the_redis_key, the_token)?;
        self.redis_jwt.get(the_redis_key)
    }
    fn put_key(&mut self, the_redis_key: &str, the_key: &str) -> redis::RedisResult<String> {
        self.redis_key.set(the_redis_key, the_key)?;
        
        self.redis_key.get(the_redis_key)
    }
    //ToDO: delete token from redis
    fn delete_token(&mut self, the_redis_key: &str) -> redis::RedisResult<String> {
        self.redis_jwt.del(the_redis_key)?;
        self.redis_jwt.get(the_redis_key)
    }
    fn delete_key(&mut self, the_redis_key: &str) -> redis::RedisResult<String> {
        self.redis_key.del(the_redis_key)?;
        self.redis_key.get(the_redis_key)
    }
}
impl RedisImplem for RedisMockProcessor{
    fn find_key(&mut self, the_key:&str)-> redis::RedisResult<String> {
        todo!()
    }

    fn find_token(&mut self, thetoken: &str) -> redis::RedisResult<String> {
        todo!()
    }

    fn put_key(&mut self, the_redis_key: &str, the_key: &str) -> redis::RedisResult<String>  {
        todo!()
    }

    fn put_token(&mut self, the_redis_key: &str, the_token: &str) -> redis::RedisResult<String>  {
        todo!()
    }

    fn delete_key(&mut self, the_redis_key: &str) -> redis::RedisResult<String> {
        todo!()
    }

    fn delete_token(&mut self, the_redis_key: &str) -> redis::RedisResult<String> {
        todo!()
    }
}
impl RedisRealProcessor{
    pub(crate) fn new(hostname: &str, port: u16) -> Self {
        let nodes = format!("redis://{}:{}/9", hostname, port).to_string();
        // let nodes = vec![format!("redis://{}:{}/9", hostname, port).to_string()];
        let redis_key_client = redis::Client::open(nodes).unwrap();
        let redis_key = redis_key_client.get_connection().unwrap();
        let nodes = format!("redis://{}:{}/10", hostname, port).to_string();
        // let nodes = vec![format!("redis://{}:{}/10", hostname, port).to_string()];
        let redis_jwt_client = redis::Client::open(nodes).unwrap();
        let redis_jwt = redis_jwt_client.get_connection().unwrap();
        Self { redis_key, redis_jwt }
    }
}
