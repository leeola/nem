use moxie_dom::*;

#[topo::aware]
pub fn simple_list() {
  let items = vec!["foo", "bar"];
  moxie::mox! {
    <ul>{
      for item in items {
        moxie::mox!(<li>{% "{}", item }</li>)
      }
    }</ul>
  }
}
