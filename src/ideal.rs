// I think this would be my ideal API for representing graphql interfaces in Rust

struct MyObj;

#[async_graphql::Object]
impl MyObj {
  async fn value_c(&self) -> i32 {
    3
  }
}

// Would the macro go on trait or impl?
#[async_graphql::Interface]
trait InterfaceA {
  async fn value_a(&self) -> i32;
}

// Would the macro go on trait or impl?
#[async_graphql::Interface]
trait InterfaceB {
  async fn value_b(&self) -> i32;
}

// Maybe it would go on both?
#[async_graphql::Interface]
impl InterfaceA for MyObj {
  #[field]
  async fn value_a(&self) -> i32 {
    1
  }
}

// Maybe it would go on both?
#[async_graphql::Interface]
impl InterfaceB for MyObj {
  #[field]
  async fn value_b(&self) -> i32 {
    2
  }
}

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
  #[field]
  async fn my_objs(&self, _ctx: &Context<'_>) -> impl InterfaceA + impl InterfaceB {
    (MyObj {}).into()
  }
}

#[tokio::main]
async fn main() {
  let schema = Schema::new(QueryRoot, EmptyMutation, EmptySubscription);
  let res = schema
    .execute(
      "{
        myObjs {
          ... on InterfaceA {
            valueA
          }
          ... on InterfaceB {
            valueB
          }
          ... on MyObj {
            valueC
          }
        }
      }",
    )
    .await;
  match res {
    Ok(success) => println!("{:?}", success.data),
    Err(Error::Rule { errors }) => println!("{:?}", errors),
    _ => println!("different error"),
  }
}
