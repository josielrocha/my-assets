use serde::{Deserialize, Serializer, Deserializer};

pub fn serialize<S>(
  value: &f64,
  serializer: S,
) -> Result<S::Ok, S::Error>
where
  S: Serializer,
{
  let s = value.to_string().replace(".", ",");
  serializer.serialize_str(&s)
}

pub fn deserialize<'de, D>(
  deserializer: D,
) -> Result<f64, D::Error>
where
  D: Deserializer<'de>,
{
  let s = String::deserialize(deserializer)?;
  let n: f64 = s.trim().replace(",", ".").parse().unwrap();
  Ok(n)
}
