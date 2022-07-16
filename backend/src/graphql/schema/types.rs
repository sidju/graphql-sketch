use super::*;

// By using this type the loader knows to load a spell when triggered
pub type SpellName = String;

#[derive(Clone, Default, Serialize, Deserialize, SimpleObject)]
#[serde(default)]
pub struct School {
  pub school: String,
  pub subschool: Option<String>,
  pub descriptors: Vec<String>,
}
#[derive(Clone, Default, Serialize, Deserialize, SimpleObject)]
#[serde(default)]
pub struct Components {
  pub verbal: bool,
  pub somatic: bool,
  pub material: Option<String>,
  pub focus: Option<String>,
  pub divine_focus: bool,
}
#[derive(Clone, Default, Serialize, Deserialize, SimpleObject)]
#[serde(default)]
pub struct Effect {
  pub range: String,
  pub area: Option<String>,
  pub target: Option<String>,
  pub duration: String,
  pub dismissible: bool,
}
#[derive(Clone, Default, Serialize, Deserialize, SimpleObject)]
#[serde(default)]
pub struct SavingThrow {
  pub fortitude: bool,
  pub reflex: bool,
  pub will: bool,
  pub description: Option<String>,
}
#[derive(Clone, Default, Serialize, Deserialize, SimpleObject)]
#[serde(default)]
pub struct SpellResistance {
  pub applies: bool,
  pub description: Option<String>,
}
#[derive(Clone, Default, Serialize, Deserialize, SimpleObject)]
#[serde(default)]
pub struct Spell {
  pub name: String, // Using spell name here fills no purpose
  pub source_book: String,
  pub url: String, // To relevant wiki page
  pub school: School,
  pub classes: HashMap<String, u8>,
  pub casting_time: String,
  pub components: Components,
  pub effect: Effect,
  pub saving_throw: SavingThrow,
  pub spell_resistance: SpellResistance,
  pub description: String,
  // Should not be null and require option, either empty list or not present
  pub related_spell_names: Option<Vec<SpellName>>,
}

/// Uses mongodb's regex engine
#[derive(Clone, Default, Serialize, Deserialize, InputObject)]
pub struct SpellFilter {
  pub name_regex: Option<String>,
  pub school_regex: Option<String>,
  pub subschool_regex: Option<String>,
  pub descriptor_regex: Option<String>,
  pub class_regex: Option<String>,
  pub class_level: Option<u8>,
  pub casting_time_unit_regex: Option<String>,
  pub verbal: Option<bool>,
  pub somatic: Option<bool>,
  pub divine_focus: Option<bool>,
  pub focus_regex: Option<String>,
  pub material_regex: Option<String>,
  pub range_regex: Option<String>,
  pub area_regex: Option<String>,
  pub target_regex: Option<String>,
  pub duration_regex: Option<String>,
  pub dismissable: Option<bool>,
  pub fortitude: Option<bool>,
  pub dexterity: Option<bool>,
  pub will: Option<bool>,
  pub save_description_regex: Option<String>,
  pub spell_resistance: Option<bool>,
  pub description: Option<String>,
}
