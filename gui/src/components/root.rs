use moxie_dom::*;

#[topo::aware]
pub fn test() {
  moxie::mox! {
    <div>
      "wee"
    </div>
  }
}

#[topo::aware]
pub fn root() {
  moxie::mox! {
    <div>
      <h1>"foo"</h1>
    </div>
  }
}
