use {moxie, moxie_dom::*};

#[topo::nested]
pub fn mox_test() {
  let items = vec!["foo", "bar"];
  moxie::mox! {
    <ul>{
      for item in items {
        moxie::mox!(<li>{% "{}", item }</li>)
      }
    }</ul>
  }
}
