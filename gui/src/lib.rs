use {moxie, moxie_dom::*};

#[topo::nested]
pub fn base_layout() {
  moxie::mox! {
    <div>
    <header role="banner">
      <nav role="navigation">
        <h1><a href="/">"Nem"</a></h1>
      </nav>
    </header>
    </div>
  }
}

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
