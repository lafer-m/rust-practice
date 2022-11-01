// We’ll implement a blog post workflow in an incremental way. The blog’s final functionality will look like this:

// A blog post starts as an empty draft.
// When the draft is done, a review of the post is requested.
// When the post is approved, it gets published.
// Only published blog posts return content to print, so unapproved posts can’t accidentally be published.



//     let mut post = Post::new();

//     post.add_text("I ate a salad for lunch today");
//     assert_eq!("", post.content());

//     post.request_review();
//     assert_eq!("", post.content());

//     post.approve();
//     assert_eq!("I ate a salad for lunch today", post.content());


pub struct Post {
    state: Option<Box<dyn State>>,
    contents: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            contents: String::from(""),
        }
    }
    pub fn add_text(&mut self,s: &str) {
        self.contents.push_str(s)
    }
    pub fn request_review(&mut self) {
       if let Some(s) = self.state.take() {
           self.state = Some(s.request_review() )
       }
    }
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }

    pub fn contents(&self) -> &str {
        self.state.as_ref().unwrap().contents(&self)
    }


    
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn contents<'a>(&self, post: &'a Post) -> &'a str;
}
// draft => review => published

struct Draft {}
impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(Review {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn contents<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Review {}
impl State for Review {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
    fn contents<'a>(&self, post: &'a Post) -> &'a str {
        r#""#
    }

}

struct Published {}
impl State for Published {
   fn request_review(self: Box<Self>) -> Box<dyn State> {
       self
   }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn contents<'a>(&self, post: &'a Post) -> &'a str {
        &post.contents
    }
}


#[test]
fn test_state_change() {
    let mut post = Post::new();
    post.add_text("test contents");
    println!("no {}", post.contents());
    post.request_review();
    println!("no {}", post.contents());
    post.approve();
    println!("yes {}", post.contents());
}




