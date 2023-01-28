use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Project {
    #[serde(rename = "$value")]
    props: Vec<Props>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct PropertyGroup {
    #[serde(rename = "$value")]
    props: Vec<GProps>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct ItemGroup {
    #[serde(rename = "$value")]
    item_group: Vec<PackageReference>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct PackageReference {
    #[serde(rename = "@Version")]
    version: String,
    #[serde(rename = "@Include")]
    include: String,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub enum Props {
    PropertyGroup(PropertyGroup),
    ItemGroup(ItemGroup),
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub enum GProps {
    TargetFramework(String),
    Nullable(String),
    ImplicitUsings(String),
    RootNamespace(String),
}
