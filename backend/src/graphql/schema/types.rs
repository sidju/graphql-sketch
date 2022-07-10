use super::*;

pub type SpellName = String;
pub type SpellId = i64;

#[derive(Clone, Serialize, Deserialize, SimpleObject)]
pub struct Spell {
  pub name: String,
  pub school: String,
  pub subschool: String,
  pub descriptors: String,
  pub class_levels: HashMap<String, u8>,
  pub casting_time: String,
  pub components: String,
  pub range: String,
  pub area: String,
  pub target: String,
  pub duration: String,
  pub effect: String,
  pub saving_throw: String,
  pub spell_resistance: bool,
  pub description: String,
  pub source_book: String,
  pub link: String,
}
