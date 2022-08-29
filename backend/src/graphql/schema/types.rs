use super::*;

// By using this type the loader knows to load a spell when triggered
pub type SpellName = String;

#[derive(Clone, Default, Serialize, Deserialize, SimpleObject)]
#[serde(default)]
#[graphql(rename_fields = "snake_case")]
pub struct School {
  pub school: String,
  pub subschool: Option<String>,
  pub descriptors: Option<Vec<String>>,
}
#[derive(Clone, Default, Serialize, Deserialize, SimpleObject)]
#[serde(default)]
#[graphql(rename_fields = "snake_case")]
pub struct Components {
  pub verbal: bool,
  pub somatic: bool,
  pub material: Option<String>,
  pub focus: Option<String>,
  pub divine_focus: bool,
}
#[derive(Clone, Default, Serialize, Deserialize, SimpleObject)]
#[serde(default)]
#[graphql(rename_fields = "snake_case")]
pub struct Effect {
  pub range: String,
  pub area: Option<String>,
  pub target: Option<String>,
  pub duration: String,
  pub dismissible: bool,
}
#[derive(Clone, Default, Serialize, Deserialize, SimpleObject)]
#[serde(default)]
#[graphql(rename_fields = "snake_case")]
pub struct SavingThrow {
  pub fortitude: bool,
  pub reflex: bool,
  pub will: bool,
  pub description: Option<String>,
}
#[derive(Clone, Default, Serialize, Deserialize, SimpleObject)]
#[serde(default)]
#[graphql(rename_fields = "snake_case")]
pub struct SpellResistance {
  pub applies: bool,
  pub description: Option<String>,
}
#[derive(Clone, Default, Serialize, Deserialize, SimpleObject)]
#[serde(default)]
#[graphql(rename_fields = "snake_case")]
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
#[serde(default)]
#[graphql(rename_fields = "snake_case")]
pub struct SpellFilter {
  pub name_regex: String,
  pub school_regex: String,
  /// To search for spells specifically without subschool use null/None
  #[graphql(default_with = "Some(String::default())")]
  pub subschool_regex: Option<String>,
  pub descriptor_regex: String,
  /// If given excludes spells with a descriptor matching
  pub descriptor_nregex: Option<String>,
  /// Spells matching class and spell level are returned,
  pub class_regex: String,
  /// Combines with class_regex if given to require spell level for that class
  /// null/None matches all levels
  pub class_spell_level: Option<u8>,
  pub casting_time_regex: String,
  pub verbal: Option<bool>,
  pub somatic: Option<bool>,
  pub divine_focus: Option<bool>,
  /// To search for spells specifically without focus use null/None
  pub focus_regex: Option<String>,
  /// To search for spells specifically without materials use null/None
  pub material_regex: Option<String>,
  pub range_regex: String,
  pub area_regex: String,
  pub target_regex: String,
  pub duration_regex: String,
  pub dismissable: Option<bool>,
  pub fortitude: Option<bool>,
  pub dexterity: Option<bool>,
  pub will: Option<bool>,
  pub save_description_regex: String,
  pub spell_resistance: Option<bool>,
  pub description: String,
}
