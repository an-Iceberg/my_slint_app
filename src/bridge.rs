use crate::BridgeUI;

pub(crate) struct BridgeBackend<'a>
{
  bridge: &'a BridgeUI<'a>,
}

impl<'a> BridgeBackend<'a>
{
  pub fn new(bridge: &'a BridgeUI) -> Self
  {
    return Self { bridge };
  }
}
