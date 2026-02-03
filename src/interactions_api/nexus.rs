use std::{collections::HashMap, path::Path};

use reqwest::{Client, header::HeaderValue};

/// an enum for holding links to dependencies and classifying them as nexus and off-site dependencies
pub enum DepType {
    Nexus(String),
    OffSite(String)
}

/// A structure used for holding data gathered from an nxm link
#[allow(unused)]
pub struct UrlData {
    game_name: String,
    mod_id: String,
    file_id: String,
    key: String,
    expires: String,
    user_id: String
}

// lets you easily turn a HashMap<&str,&str> into a UrlData as long as the HashMap has the proper indicies
impl From<HashMap<&str,&str>> for UrlData {
    fn from(value: HashMap<&str,&str>) -> Self {
        Self {
            game_name: value["game_name"].to_string(),
            mod_id: value["mod_id"].to_string(),
            file_id: value["file_id"].to_string(),
            key: value["key"].to_string(),
            expires: value["expires"].to_string(),
            user_id: value["user_id"].to_string()
        }
    }
}

impl UrlData {
    /// formats the currently held data into an api interaction that when connected to returns a download link to the referenced file 
    pub fn get_url(&self) -> String {
        // literally just a format call
        format!("https://api.nexusmods.com/v1/games/{}/mods/{}/files/{}/download_link.json?key={}&expires={}",self.game_name,self.mod_id,self.file_id,self.key,self.expires)
    }
}

/// takes an nxm link and deconstructs it into the relevant and important pieces of data for formatting a related api call
pub fn deconstruct_nxm<'life>(link: &'life str) -> UrlData {
    // format seems to be as follows:
    // nxm://{game_namespace}/mods/{mod_id}/files/{file_id}?key={key}&expires={expiry_time}&user_id={user_id}

    //create data container to return later
    let mut data: HashMap<&str,&str> = HashMap::new();

    // extrapolate from path
    {
        // split data by slashes and cache
        let temp_data = link.split_at(6).1.split('/').collect::<Vec<&str>>();
    
        // assign relevant components
        data.insert("game_name", temp_data[0]);
        data.insert("mod_id", temp_data[2]);
        data.insert("file_id", temp_data[4].split('?').collect::<Vec<&str>>()[0]);
    }

    // extrapolate from query string
    {
        // split data by the question mark and cache
        let temp_data = link.split('?').skip(1).collect::<Vec<&str>>()[0].split('&').collect::<Vec<&str>>();
    
        // assign relevant components
        data.insert("key", temp_data[0].split_at(4).1);
        data.insert("expires", temp_data[1].split_at(8).1);
        data.insert("user_id", temp_data[2].split_at(8).1);
    }

    return data.into();
}

/// uses an api key, nxm link, and a target path to download a mod from an nxm link
pub async fn download_mod(api_key: &str, nxm_link: &str, path: impl AsRef<Path>) {

    use std::{io::Write, str::FromStr};

    use reqwest::{Client, header::HeaderValue};

    // download url constructed with the specs from a deconstructed nxm link
    let url = deconstruct_nxm(nxm_link).get_url();

    // new client for interacting with the nexus api
    let client = Client::new();

    // get the download url from the nexus api using the api key
    let response = client.get(url)
        .header("apikey", HeaderValue::from_str(api_key).expect("Expected to be able to read header from str"))
        .send()
        .await
        .expect("Expected to be able to send data to site");

    // this block just extracts the url from the json response
    let text = response.text().await.expect("Expected to be able to convert response to text");
    let json = serde_json::Value::from_str(&text.as_str()).expect("Expected to be able to parse json data");
    let binding = json[0]["URI"].to_string();
    let uri = binding.split_at(1).1;
    let uri = uri.split_at(uri.len()-1).0;
    
    // prints the url in debug mode
    #[cfg(debug_assertions)]
    println!("Response: {}", uri);

    // get the downloaded data as bytes from the download url
    let bytes = client.get(uri)
        .send()
        .await
        .expect("Expected to be able to get the file from the download link")
        .bytes()
        .await
        .expect("Expected to be able to get the file as bytes");

    // creates a new file at the target path
    let mut dest = std::fs::File::create(path).expect("Expected to be able to create file");

    // writes all the data to the file
    dest.write_all(&bytes).expect("Expected to be able to write bytes to the file");
}


///// !!warning!!
///// this implementation is so jank, but nexus doesn't have a way to get dependencies via their api, so blame them
//pub async fn get_dependencies(game_id: &str, mod_id: i32) {//-> Vec<DepType> {
//    let mod_page = format!("https://www.nexusmods.com/{game_id}/mods/{mod_id}?tab=description");
//
//    let client = Client::new();
//
//    let response = client.get(mod_page)
//        .send()
//        .await
//        .expect("Expected to be able to get the mod's home page")
//        .text()
//        .await
//        .expect("Expected to be able to turn response into text");
//
//    println!("{response}")
//}