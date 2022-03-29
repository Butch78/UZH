use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Municipality {
    #[serde(rename = "MunicipalityNumber")]
    municipality_number: u32,
    #[serde(rename = "MunicipalityName")]
    municipality_name: String,

    #[serde(rename = "Canton")]
    #[serde(default = "default_canton")]
    canton: String,

    #[serde(rename = "Country")]
    country: String,

    #[serde(rename = "Scenario1_RoofsOnly_PotentialSolarElectricity_GWh")]
    scenario1_roofs_only_potential_solar_electricity_gwh: f32,

    #[serde(rename = "Scenario2_RoofsOnly_PotentialSolarElectricity_GWh")]
    scenario2_roofs_only_potential_solar_electricity_gwh: f32,

    #[serde(rename = "Scenario2_RoofsOnly_PotentialSolarHeat_GWh")]
    scenario2_roofs_only_potential_solar_heat_gwh: f32,

    // Maps the incoming Key to the correct key in the output
    #[serde(rename = "Scenario3_RoofsFacades_PotentialSolarElectricity_GWh")]
    scenario3_roofs_facades_potential_solar_electricity_gwh: f32,

    #[serde(rename = "Scenario4_RoofsFacades_PotentialSolarElectricity_GWh")]
    scenario4_roofs_facades_potential_solar_electricity_gwh: f32,

    #[serde(rename = "Scenario4_RoofsFacades_PotentialSolarHeat_GWh")]
    scenario4_roofs_facades_potential_solar_heat_gwh: f32,

    #[serde(rename = "Factsheet")]
    factsheet: String,

    #[serde(rename = "Methodology")]
    methodology: String,
}

fn default_canton() -> String {
    "Zurich".to_string()
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct SimpleMunicipality {
    name: String,
    canton: String,
    scenario3_roofs_facades_potential_solar_electricity_gwh: i32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Sending Request...");
    let res = reqwest::get("http://www.uvek-gis.admin.ch/BFE/ogd/52/Solarenergiepotenziale_Gemeinden_Daecher_und_Fassaden.json").await?;
    println!("Status: {}", res.status());
    let body = res.text().await?;

    let municipalities: Vec<Municipality> = serde_json::from_str(&body)?;


    let total =
        calculate_total_scenario3_roofs_facades_potential_solar_electricity_gwh(&municipalities);

    println!(
        "Total of Scenario3_RoofsFacades_PotentialSolarElectricity_GWh: {}",
        total
    );

    let third_largest = find_3rd_largest_scenario3(&municipalities);

    println!(
        "The 3rd largest Potential : {} and Canton {}",
        third_largest.name, third_largest.canton
    );

    Ok(())
}

// Calculate total of Scenario3_RoofsFacades_PotentialSolarElectricity_GWh
fn calculate_total_scenario3_roofs_facades_potential_solar_electricity_gwh(
    municipalities: &Vec<Municipality>,
) -> f32 {
    let mut total = 0.0;
    for municipality in municipalities {
        total += municipality.scenario3_roofs_facades_potential_solar_electricity_gwh;
    }
    total
}

fn find_3rd_largest_scenario3(municipalities: &Vec<Municipality>) -> SimpleMunicipality {
    // Convert to simple_municipality because you can't sort a struct by f32 in rust
    let mut simple_municipalities: Vec<SimpleMunicipality> = Vec::new();

    for municipality in municipalities {
        simple_municipalities.push(SimpleMunicipality {
            name: municipality.municipality_name.clone(),
            canton: municipality.canton.clone(),
            scenario3_roofs_facades_potential_solar_electricity_gwh: municipality
                .scenario3_roofs_facades_potential_solar_electricity_gwh
                as i32,
        });
    }

    // Sort by scenario3_roofs_facades_potential_solar_electricity_gwh
    simple_municipalities.sort_by(|a, b| {
        a.scenario3_roofs_facades_potential_solar_electricity_gwh
            .cmp(&b.scenario3_roofs_facades_potential_solar_electricity_gwh)
    });

    // Return the 3rd largest
    return simple_municipalities[simple_municipalities.len() - 3].clone();
}
