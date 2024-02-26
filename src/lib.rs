use serde::Deserialize;

pub struct Client {
    api_key: String,
    cx: String,
    reqwest: reqwest::Client,
}

impl Client {
    pub fn new(api_key: String, cx: String) -> Self {
        Client {
            api_key,
            cx,
            reqwest: reqwest::Client::builder().gzip(true).build().unwrap(),
        }
    }

    pub async fn search(&self, query: &str, start: u64) -> Result<Response, reqwest::Error> {
        let url = format!(
            "https://www.googleapis.com/customsearch/v1?key={}&cx={}&q={}&searchType=image&start={}",
            self.api_key, self.cx, query, start
        );
        let response = self.reqwest.get(&url).send().await?;
        let response = response.json::<Response>().await?;
        Ok(response)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub kind: String,
    pub url: Url,
    pub queries: Queries,
    pub context: Context,
    pub search_information: SearchInformation,
    pub items: Vec<Item>,
}

#[derive(Debug, Deserialize)]
pub struct Url {
    pub r#type: String,
    pub template: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Queries {
    pub request: Vec<Request>,
    pub next_page: Vec<Request>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub title: String,
    pub total_results: String,
    pub search_terms: String,
    pub count: i32,
    pub start_index: i32,
    pub input_encoding: String,
    pub output_encoding: String,
    pub safe: String,
    pub cx: String,
    pub search_type: String,
}

#[derive(Debug, Deserialize)]
pub struct Context {
    pub title: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchInformation {
    pub search_time: f32,
    pub formatted_search_time: String,
    pub total_results: String,
    pub formatted_total_results: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub kind: String,
    pub title: String,
    pub html_title: String,
    pub link: String,
    pub display_link: String,
    pub snippet: String,
    pub html_snippet: String,
    pub mime: String,
    pub file_format: String,
    pub image: Image,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub context_link: String,
    pub height: i32,
    pub width: i32,
    pub byte_size: i32,
    pub thumbnail_link: String,
    pub thumbnail_height: i32,
    pub thumbnail_width: i32,
}

#[cfg(test)]
mod test {

    const example_response: &str = r#"{
  "kind": "customsearch#search",
  "url": {
    "type": "application/json",
    "template": "https://www.googleapis.com/customsearch/v1?q={searchTerms}&num={count?}&start={startIndex?}&lr={language?}&safe={safe?}&cx={cx?}&sort={sort?}&filter={filter?}&gl={gl?}&cr={cr?}&googlehost={googleHost?}&c2coff={disableCnTwTranslation?}&hq={hq?}&hl={hl?}&siteSearch={siteSearch?}&siteSearchFilter={siteSearchFilter?}&exactTerms={exactTerms?}&excludeTerms={excludeTerms?}&linkSite={linkSite?}&orTerms={orTerms?}&dateRestrict={dateRestrict?}&lowRange={lowRange?}&highRange={highRange?}&searchType={searchType}&fileType={fileType?}&rights={rights?}&imgSize={imgSize?}&imgType={imgType?}&imgColorType={imgColorType?}&imgDominantColor={imgDominantColor?}&alt=json"
  },
  "queries": {
    "request": [
      {
        "title": "Google Custom Search - hedgehog",
        "totalResults": "3800000000",
        "searchTerms": "hedgehog",
        "count": 10,
        "startIndex": 1,
        "inputEncoding": "utf8",
        "outputEncoding": "utf8",
        "safe": "off",
        "cx": "8677ac224fbcc40b7",
        "searchType": "image"
      }
    ],
    "nextPage": [
      {
        "title": "Google Custom Search - hedgehog",
        "totalResults": "3800000000",
        "searchTerms": "hedgehog",
        "count": 10,
        "startIndex": 11,
        "inputEncoding": "utf8",
        "outputEncoding": "utf8",
        "safe": "off",
        "cx": "8677ac224fbcc40b7",
        "searchType": "image"
      }
    ]
  },
  "context": {
    "title": "Image"
  },
  "searchInformation": {
    "searchTime": 0.360723,
    "formattedSearchTime": "0.36",
    "totalResults": "3800000000",
    "formattedTotalResults": "3,800,000,000"
  },
  "items": [
    {
      "kind": "customsearch#result",
      "title": "Hedgehog",
      "htmlTitle": "\u003cb\u003eHedgehog\u003c/b\u003e",
      "link": "https://i.natgeofe.com/k/dfc55ac8-a221-4390-b440-72ebfe2bfc39/hedgehog-staring_2x3.jpg",
      "displayLink": "kids.nationalgeographic.com",
      "snippet": "hedgehog-staring_2x3.jpg",
      "htmlSnippet": "\u003cb\u003ehedgehog\u003c/b\u003e-staring_2x3.jpg",
      "mime": "image/jpeg",
      "fileFormat": "image/jpeg",
      "image": {
        "contextLink": "https://kids.nationalgeographic.com/animals/mammals/facts/hedgehog",
        "height": 3072,
        "width": 2048,
        "byteSize": 669144,
        "thumbnailLink": "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQrqAYZ8BWOBmhHnM_d_ICKStp9DMbIqdAHawhdFa7b55gRHsVLId26Zjs&s",
        "thumbnailHeight": 150,
        "thumbnailWidth": 100
      }
    },
    {
      "kind": "customsearch#result",
      "title": "A new species of hedgehog stands out for its short spikes",
      "htmlTitle": "A new species of \u003cb\u003ehedgehog\u003c/b\u003e stands out for its short spikes",
      "link": "https://i0.wp.com/www.sciencenews.org/wp-content/uploads/2023/12/120723_mt_hedgehog_feat.jpg?fit=1030%2C580&ssl=1",
      "displayLink": "www.sciencenews.org",
      "snippet": "A new species of hedgehog stands out for its short spikes",
      "htmlSnippet": "A new species of \u003cb\u003ehedgehog\u003c/b\u003e stands out for its short spikes",
      "mime": "image/jpeg",
      "fileFormat": "image/jpeg",
      "image": {
        "contextLink": "https://www.sciencenews.org/article/new-hedgehog-species-short-spikes",
        "height": 579,
        "width": 1029,
        "byteSize": 152184,
        "thumbnailLink": "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQHF3RbzuvNxjGcPG9QbvIcGaSn5cii5B9HA2Y8_rNdRL944mthvGGKTjQ&s",
        "thumbnailHeight": 84,
        "thumbnailWidth": 150
      }
    },
    {
      "kind": "customsearch#result",
      "title": "Caring For Your Pet Hedgehog | Pender Veterinary Centre",
      "htmlTitle": "Caring For Your Pet \u003cb\u003eHedgehog\u003c/b\u003e | Pender Veterinary Centre",
      "link": "https://images.ctfassets.net/rt5zmd3ipxai/4Z1RIc1p8375Fb1JPvVhcg/f39b175c5b030bd6e2df81fa1b9fb0de/ServiceCards-Hedgehog.png?fit=fill&fm=webp&h=578&w=1070&q=72,%20https://images.ctfassets.net/rt5zmd3ipxai/4Z1RIc1p8375Fb1JPvVhcg/f39b175c5b030bd6e2df81fa1b9fb0de/ServiceCards-Hedgehog.png?fit=fill&fm=webp&h=1156&w=2140&q=72",
      "displayLink": "www.pendervet.com",
      "snippet": "Caring For Your Pet Hedgehog | Pender Veterinary Centre",
      "htmlSnippet": "Caring For Your Pet \u003cb\u003eHedgehog\u003c/b\u003e | Pender Veterinary Centre",
      "mime": "image/png",
      "fileFormat": "image/png",
      "image": {
        "contextLink": "https://www.pendervet.com/blog/caring-for-your-pet-hedgehog",
        "height": 1156,
        "width": 2140,
        "byteSize": 149644,
        "thumbnailLink": "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcR8VtsO-plzaiqoXD0mwroHyQ-in8f--4kKhmC2ZujBGql8qhtGR2M1Hg&s",
        "thumbnailHeight": 81,
        "thumbnailWidth": 150
      }
    },
    {
      "kind": "customsearch#result",
      "title": "Hedgehog Pets Cute But Challenging - Veterinary Medicine at Illinois",
      "htmlTitle": "\u003cb\u003eHedgehog\u003c/b\u003e Pets Cute But Challenging - Veterinary Medicine at Illinois",
      "link": "https://vetmed.illinois.edu/wp-content/uploads/2021/04/pc-keller-hedgehog.jpg",
      "displayLink": "vetmed.illinois.edu",
      "snippet": "Hedgehog Pets Cute But Challenging - Veterinary Medicine at Illinois",
      "htmlSnippet": "\u003cb\u003eHedgehog\u003c/b\u003e Pets Cute But Challenging - Veterinary Medicine at Illinois",
      "mime": "image/jpeg",
      "fileFormat": "image/jpeg",
      "image": {
        "contextLink": "https://vetmed.illinois.edu/pet-health-columns/hedgehog-pets/",
        "height": 370,
        "width": 615,
        "byteSize": 58367,
        "thumbnailLink": "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcRUVXqzTdmzRPZsjjgme7tb0Ap3uZUDPaFz0UTclQlh29IR6Rhp2OCFew&s",
        "thumbnailHeight": 82,
        "thumbnailWidth": 136
      }
    },
    {
      "kind": "customsearch#result",
      "title": "Hedgehog - Wikipedia",
      "htmlTitle": "\u003cb\u003eHedgehog\u003c/b\u003e - Wikipedia",
      "link": "https://upload.wikimedia.org/wikipedia/commons/7/72/Igel.JPG",
      "displayLink": "en.wikipedia.org",
      "snippet": "Hedgehog - Wikipedia",
      "htmlSnippet": "\u003cb\u003eHedgehog\u003c/b\u003e - Wikipedia",
      "mime": "image/jpeg",
      "fileFormat": "image/jpeg",
      "image": {
        "contextLink": "https://en.wikipedia.org/wiki/Hedgehog",
        "height": 846,
        "width": 1075,
        "byteSize": 945610,
        "thumbnailLink": "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcRrTGjxj6-5ixj9IDntxM-gX2wpTbvEth5VfsyMeQmOWTUZ-MlS10qlgTU&s",
        "thumbnailHeight": 118,
        "thumbnailWidth": 150
      }
    },
    {
      "kind": "customsearch#result",
      "title": "World's oldest European hedgehog could provide hope for the future ...",
      "htmlTitle": "World&#39;s oldest European \u003cb\u003ehedgehog\u003c/b\u003e could provide hope for the future ...",
      "link": "https://static.euronews.com/articles/stories/07/41/58/32/1440x810_cmsv2_27db9763-596e-5480-81d9-8b94ce0000ea-7415832.jpg",
      "displayLink": "www.euronews.com",
      "snippet": "World's oldest European hedgehog could provide hope for the future ...",
      "htmlSnippet": "World&#39;s oldest European \u003cb\u003ehedgehog\u003c/b\u003e could provide hope for the future ...",
      "mime": "image/jpeg",
      "fileFormat": "image/jpeg",
      "image": {
        "contextLink": "https://www.euronews.com/green/2023/02/20/worlds-oldest-european-hedgehog-could-provide-hope-for-the-future-of-the-species",
        "height": 810,
        "width": 1440,
        "byteSize": 199084,
        "thumbnailLink": "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQ0b7AjkhwjAB2ATXPYO5rwdRlEs5XL7qGECVoxCk7CoeCTgltSr1HZsw&s",
        "thumbnailHeight": 84,
        "thumbnailWidth": 150
      }
    },
    {
      "kind": "customsearch#result",
      "title": "Hedgehog Facts and Considerations | South Wilton Vet",
      "htmlTitle": "\u003cb\u003eHedgehog\u003c/b\u003e Facts and Considerations | South Wilton Vet",
      "link": "https://www.southwiltonvet.com/files/HedgehogSouthWiltonVet.jpeg",
      "displayLink": "www.southwiltonvet.com",
      "snippet": "Hedgehog Facts and Considerations | South Wilton Vet",
      "htmlSnippet": "\u003cb\u003eHedgehog\u003c/b\u003e Facts and Considerations | South Wilton Vet",
      "mime": "image/jpeg",
      "fileFormat": "image/jpeg",
      "image": {
        "contextLink": "https://www.southwiltonvet.com/site/blog-fairfield-county-vet/2020/12/15/hedgehog-facts-considerations",
        "height": 2733,
        "width": 4100,
        "byteSize": 687248,
        "thumbnailLink": "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcRkkky4YxgcJZ9jV1RAWGQEupZt1yNwuCpiJVMiK39o5ITJsmZLnlOTI-Q&s",
        "thumbnailHeight": 100,
        "thumbnailWidth": 150
      }
    },
    {
      "kind": "customsearch#result",
      "title": "Long-eared hedgehog - Wikipedia",
      "htmlTitle": "Long-eared \u003cb\u003ehedgehog\u003c/b\u003e - Wikipedia",
      "link": "https://upload.wikimedia.org/wikipedia/commons/6/66/Hemiechinus_auritus%3B_Baikonur_09.jpg",
      "displayLink": "en.wikipedia.org",
      "snippet": "Long-eared hedgehog - Wikipedia",
      "htmlSnippet": "Long-eared \u003cb\u003ehedgehog\u003c/b\u003e - Wikipedia",
      "mime": "image/jpeg",
      "fileFormat": "image/jpeg",
      "image": {
        "contextLink": "https://en.wikipedia.org/wiki/Long-eared_hedgehog",
        "height": 1800,
        "width": 2400,
        "byteSize": 2046500,
        "thumbnailLink": "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQ3KvXSqk7RejyLxNsy-8BXPh1wzEwE4fVxmhCk7uIyicxY9ZtE3sNcNA&s",
        "thumbnailHeight": 113,
        "thumbnailWidth": 150
      }
    },
    {
      "kind": "customsearch#result",
      "title": "Hedgehog, African Pygmy - Louisville Zoo",
      "htmlTitle": "\u003cb\u003eHedgehog\u003c/b\u003e, African Pygmy - Louisville Zoo",
      "link": "https://louisvillezoo.org/wp-content/uploads/2014/12/hedheog.png",
      "displayLink": "louisvillezoo.org",
      "snippet": "Hedgehog, African Pygmy - Louisville Zoo",
      "htmlSnippet": "\u003cb\u003eHedgehog\u003c/b\u003e, African Pygmy - Louisville Zoo",
      "mime": "image/png",
      "fileFormat": "image/png",
      "image": {
        "contextLink": "https://louisvillezoo.org/animalsandplants/hedgehog-african-pygmy/",
        "height": 400,
        "width": 870,
        "byteSize": 581472,
        "thumbnailLink": "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcRL7T-Qc2lBevPuAYOYbr28AS_CNkxo2-p8pb8uO9D36y5x1Aa_YUgQ5Fk&s",
        "thumbnailHeight": 67,
        "thumbnailWidth": 145
      }
    },
    {
      "kind": "customsearch#result",
      "title": "Hedgehog",
      "htmlTitle": "\u003cb\u003eHedgehog\u003c/b\u003e",
      "link": "https://i.natgeofe.com/k/dfc55ac8-a221-4390-b440-72ebfe2bfc39/hedgehog-staring_3x2.jpg",
      "displayLink": "kids.nationalgeographic.com",
      "snippet": "hedgehog-staring_3x2.jpg",
      "htmlSnippet": "\u003cb\u003ehedgehog\u003c/b\u003e-staring_3x2.jpg",
      "mime": "image/jpeg",
      "fileFormat": "image/jpeg",
      "image": {
        "contextLink": "https://kids.nationalgeographic.com/animals/mammals/facts/hedgehog",
        "height": 2048,
        "width": 3072,
        "byteSize": 541643,
        "thumbnailLink": "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcScFfAJ7kw2SmJSdi83Wl7KlilPWYBw-8-WGXPeV4cLnBvlc-c2xy25BQ&s",
        "thumbnailHeight": 100,
        "thumbnailWidth": 150
      }
    }
  ]
}"#;

    #[test]
    fn test_deserialize() {
        let response: super::Response = serde_json::from_str(example_response).unwrap();
        assert_eq!(response.kind, "customsearch#search");
        assert_eq!(response.url.r#type, "application/json");
        assert_eq!(
            response.queries.request[0].title,
            "Google Custom Search - hedgehog"
        );
        assert_eq!(response.items.len(), 10);
    }
}
