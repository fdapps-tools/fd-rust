mod storage;

#[derive(Debug)]

pub struct Node {
  id: i32,
  host: String,
  lastcheck: String,
  hash: String,
}

pub fn add(node Node) -> Result<()> {
  storage::node_add(node)
  Ok(())
}

pub fn list() -> Result<()> {
  storage::node_list()
  Ok(())
}

pub fn init() -> Result<()> {}
pub fn getNodeList() -> Result<()> {}
pub fn insertNode() -> Result<()> {}
pub fn checkNodesIsUp() -> Result<()> {}
pub fn checkHostIsUp() -> Result<()> {}
pub fn syncJoinRequests() -> Result<()> {}
pub fn setApprovalOrInapproval() -> Result<()> {}
pub fn broadcastFile() -> Result<()> {}