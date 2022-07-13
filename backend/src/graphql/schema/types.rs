use super::*;

// By using this type the loader knows to load a spell when triggered
pub type SpellName = String;

#[derive(Clone, Default, Serialize, Deserialize, SimpleObject)]
pub struct School {
  pub school: String,
  pub subschool: Option<String>,
  pub descriptors: Vec<String>,
}
#[derive(Clone, Default, Serialize, Deserialize, SimpleObject)]
pub struct CastingTime {
  pub unit: String,
  pub value: u32,
}
#[derive(Clone, Default, Serialize, Deserialize, SimpleObject)]
pub struct Components {
  pub verbal: bool,
  pub somatic: bool,
  pub material: Option<String>,
  pub focus: Option<String>,
  pub divine_focus: bool,
}
#[derive(Clone, Default, Serialize, Deserialize, SimpleObject)]
pub struct Effect {
  pub range: String,
  pub area: Option<String>,
  pub target: String,
  pub duration: String,
  pub dissmissible: bool,
}
#[derive(Clone, Default, Serialize, Deserialize, SimpleObject)]
pub struct SavingThrow {
  pub fortitude: bool,
  pub reflex: bool,
  pub will: bool,
  pub description: String,
}
#[derive(Clone, Default, Serialize, Deserialize, SimpleObject)]
pub struct SpellResistance {
  pub applies: bool,
  pub description: String,
}
#[derive(Clone, Default, Serialize, Deserialize, SimpleObject)]
pub struct Spell {
  pub name: String, // Using spell name here fills no purpose
  pub source_book: String,
  pub url: String, // To relevant wiki page
  pub schools: School,
  pub classes: HashMap<String, u8>,
  pub casting_time: CastingTime,
  pub components: Components,
  pub effect: Effect,
  pub saving_throw: SavingThrow,
  pub spell_resistance: SpellResistance,
  pub description: String,
  pub related_spell_names: Vec<SpellName>,
}
