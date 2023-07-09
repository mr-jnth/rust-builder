use crate::custom::*;

#[derive(Debug)]
pub struct Tweet {
    pub user: String,
    pub subject: String,
    pub body: String,
}

#[derive(Default)]
pub struct TweetBuilder {
    pub user: Option<String>,
    pub subject: Option<String>,
    pub body: Option<String>,
}

impl TweetBuilder {
    pub fn new() -> Self {
        TweetBuilder::default()
    }

    pub fn user(&mut self, user: impl Into<String>) -> &mut Self {
        self.user = Some(user.into());
        self
    }

    pub fn subject(&mut self, subject: impl Into<String>) -> &mut Self {
        self.subject = Some(subject.into());
        self
    }

    pub fn body(&mut self, body: impl Into<String>) -> &mut Self {
        self.body = Some(body.into());
        self
    }

    pub fn build(&self) -> Result<Tweet> {
        let user = self.user
        .as_ref()
        .cloned()
        .unwrap_or_else(|| "Anonymous".to_string());
        
        let Some(body) = self.body.as_ref().cloned() else {
            return Err(Error::NoBody("Add a body".to_string()));
        };
        
        let Some(subject) = self.subject.as_ref().cloned() else {
            return Err(Error::NoSubject("Add a subject".to_string()));
        };

        Ok(
            Tweet {
                user,
                subject: subject.to_string(),
                body,
            }
        )
    }
}