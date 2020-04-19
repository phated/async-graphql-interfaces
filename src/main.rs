use async_graphql::*;

struct MyObj;

#[async_graphql::Object]
impl MyObj {
  #[field]
  async fn value_a(&self) -> i32 {
    1
  }

  #[field]
  async fn value_b(&self) -> i32 {
    2
  }
}

#[async_graphql::Interface(field(name = "value_a", type = "i32"))]
struct InterfaceA(MyObj);

#[async_graphql::Interface(field(name = "value_b", type = "i32"))]
struct InterfaceB(MyObj);

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
  #[field]
  async fn my_objs(&self, _ctx: &Context<'_>) -> InterfaceA {
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
