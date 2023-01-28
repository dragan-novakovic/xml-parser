use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Project {
    #[serde(rename = "$value")]
    props: Vec<Props>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub enum Props {
    PropertyGroup(PropertyGroup),
    // TO-DO
    ItemGroup(),
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct PropertyGroup {
    #[serde(rename = "$value")]
    props: Vec<PGProperties>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub enum PGProperties {
    TargetFrameworks(String),
    OutputType(String),
    ImplicitUsings(String),
    RootNamespace(String),
    UseMaui(String),
    ApplicationTitle(String),
    SingleProject(String),
    ApplicationId(String),
    ApplicationIdGuid(String),
    ApplicationDisplayVersion(String),
    ApplicationVersion(String),
    SupportedOSPlatformVersion(String),
}

// <Project Sdk="Microsoft.NET.Sdk">

// 	<PropertyGroup>
// 		<TargetFrameworks>net7.0-android;net7.0-ios;net7.0-maccatalyst</TargetFrameworks>
// 		<TargetFrameworks Condition="$([MSBuild]::IsOSPlatform('windows'))">$(TargetFrameworks);net7.0-windows10.0.19041.0</TargetFrameworks>
// 		<!-- Uncomment to also build the tizen app. You will need to install tizen by following this: https://github.com/Samsung/Tizen.NET -->
// 		<!-- <TargetFrameworks>$(TargetFrameworks);net7.0-tizen</TargetFrameworks> -->
// 		<OutputType>Exe</OutputType>
// 		<RootNamespace>TodoREST</RootNamespace>
// 		<UseMaui>true</UseMaui>
// 		<SingleProject>true</SingleProject>
// 		<ImplicitUsings>enable</ImplicitUsings>

// 		<!-- Display name -->
// 		<ApplicationTitle>TodoREST</ApplicationTitle>

// 		<!-- App Identifier -->
// 		<ApplicationId>com.companyname.todorest</ApplicationId>
// 		<ApplicationIdGuid>E03E5DF4-5752-4412-BCE6-4B2BDFF9FAE6</ApplicationIdGuid>

// 		<!-- Versions -->
// 		<ApplicationDisplayVersion>1.0</ApplicationDisplayVersion>
// 		<ApplicationVersion>1</ApplicationVersion>

// 		<SupportedOSPlatformVersion Condition="$([MSBuild]::GetTargetPlatformIdentifier('$(TargetFramework)')) == 'ios'">14.2</SupportedOSPlatformVersion>
// 		<SupportedOSPlatformVersion Condition="$([MSBuild]::GetTargetPlatformIdentifier('$(TargetFramework)')) == 'maccatalyst'">14.0</SupportedOSPlatformVersion>
// 		<SupportedOSPlatformVersion Condition="$([MSBuild]::GetTargetPlatformIdentifier('$(TargetFramework)')) == 'android'">21.0</SupportedOSPlatformVersion>
// 		<SupportedOSPlatformVersion Condition="$([MSBuild]::GetTargetPlatformIdentifier('$(TargetFramework)')) == 'windows'">10.0.17763.0</SupportedOSPlatformVersion>
// 		<TargetPlatformMinVersion Condition="$([MSBuild]::GetTargetPlatformIdentifier('$(TargetFramework)')) == 'windows'">10.0.17763.0</TargetPlatformMinVersion>
// 		<SupportedOSPlatformVersion Condition="$([MSBuild]::GetTargetPlatformIdentifier('$(TargetFramework)')) == 'tizen'">6.5</SupportedOSPlatformVersion>
// 	</PropertyGroup>

// 	<ItemGroup>
// 		<!-- App Icon -->
// 		<MauiIcon Include="Resources\AppIcon\appicon.svg" ForegroundFile="Resources\AppIcon\appiconfg.svg" Color="#512BD4" />

// 		<!-- Splash Screen -->
// 		<MauiSplashScreen Include="Resources\Splash\splash.svg" Color="#512BD4" BaseSize="128,128" />

// 		<!-- Images -->
// 		<MauiImage Include="Resources\Images\*" />
// 		<MauiFont Include="Resources\Fonts\*" />

// 		<!-- Raw Assets (also remove the "Resources\Raw" prefix) -->
// 		<MauiAsset Include="Resources\Raw\**" LogicalName="%(RecursiveDir)%(Filename)%(Extension)" />
// 	</ItemGroup>

// 	<ItemGroup>
// 	  <None Remove="Platforms\Android\Resources\xml\network_security_config.xml" />
// 	  <None Remove="Resources\Images\check.png" />
// 	  <None Remove="Resources\Images\plus.png" />
// 	</ItemGroup>

// 	<ItemGroup>
// 	  <MauiXaml Update="Views\TodoItemPage.xaml">
// 	    <Generator>MSBuild:Compile</Generator>
// 	  </MauiXaml>
// 	  <MauiXaml Update="Views\TodoListPage.xaml">
// 	    <Generator>MSBuild:Compile</Generator>
// 	  </MauiXaml>
// 	</ItemGroup>

// </Project>
