use anyhow::{Context, Result};
use fantoccini::{ClientBuilder, Locator};
use scraper::{ElementRef, Html, Selector};

/********************************************************************************************
 * Datastructure
********************************************************************************************/

// Columns: Played At,	Artist,	Song,	Album
#[derive(Clone, Debug, PartialEq)]
pub struct PlaylistItem {
    played_at: String,
    artist: String,
    song: String,
    album: String,
}

/********************************************************************************************
 * Methods
********************************************************************************************/
pub async fn try_get_playlist() -> Result<String> {
    // Connecting using "native" TLS (with feature `native-tls`; on by default)
    //let c = ClientBuilder::native().connect("http://localhost:4444").await.expect("failed to connect to WebDriver");
    // Connecting using Rustls (with feature `rustls-tls`)
    let c = ClientBuilder::rustls()
        .connect("http://localhost:8888")
        .await
        .expect("failed to connect to WebDriver");

    c.goto("https://somafm.com/groovesalad/songhistory.html")
        .await?;

    let html = c.find(Locator::Css("#playinc")).await?.html(true).await?;

    c.close().await?;

    let res = html_escape::decode_html_entities(html.as_str()).to_string();

    Ok(res)
}

pub fn try_scrape_html(html_escaped_payload: &str) -> Result<Vec<PlaylistItem>> {
    let mut res = Vec::<PlaylistItem>::new();

    let fragment = Html::parse_fragment(html_escaped_payload);

    let table_selector =
        Selector::parse("table").map_err(|e| anyhow::anyhow!("An error occurred: {:#?}", e))?;
    let tbody_selector =
        Selector::parse("tbody").map_err(|e| anyhow::anyhow!("An error occurred: {:#?}", e))?;
    let tr_selector =
        Selector::parse("tr").map_err(|e| anyhow::anyhow!("An error occurred: {:#?}", e))?;
    let td_selector =
        Selector::parse("td").map_err(|e| anyhow::anyhow!("An error occurred: {:#?}", e))?;
    let a_selector =
        Selector::parse("a").map_err(|e| anyhow::anyhow!("An error occurred: {:#?}", e))?;

    let table = fragment
        .select(&table_selector)
        .next()
        .context("An error occurred")?
        .select(&tbody_selector)
        .next()
        .context("An error occurred")?;

    'next_row: for (index, row) in table.select(&tr_selector).enumerate() {
        let tds = row.select(&td_selector).collect::<Vec<_>>();
        /*
          Ignore first (heading) and second (separator) row
          Ignore any row that is tds.len() < 5
          Ignore last (separator) row -- above rule captures this rule
        */

        if index < 2 || tds.len() < 5 {
            continue 'next_row;
        }

        let get_inner = |td: ElementRef| -> Result<String> {
            let res = td
                .select(&a_selector)
                .next()
                .context("An error occurred")?
                .inner_html();

            Ok(res)
        };

        let time = html_escape::decode_html_entities(tds[0].inner_html().as_str()).to_string()[..8]
            .to_string();

        let item = PlaylistItem {
            played_at: time,
            artist: html_escape::decode_html_entities(get_inner(tds[1])?.as_str()).to_string(),
            song: html_escape::decode_html_entities(tds[2].inner_html().as_str()).to_string(),
            album: html_escape::decode_html_entities(get_inner(tds[3])?.as_str()).to_string(),
        };

        res.push(item);
    }

    Ok(res)
}



/********************************************************************************************
 * TESTS
********************************************************************************************/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        println!("Hello somafm test!");

        assert!(true)
    }

    #[tokio::test]
    #[ignore]
    async fn test_02_connect_to_somafm() {
        let res = async {
            use fantoccini::{ClientBuilder, Locator};

            // Connecting using "native" TLS (with feature `native-tls`; on by default)
            //let c = ClientBuilder::native().connect("http://localhost:4444").await.expect("failed to connect to WebDriver");
            // Connecting using Rustls (with feature `rustls-tls`)
            let c = ClientBuilder::rustls()
                .connect("http://localhost:8888")
                .await
                .expect("failed to connect to WebDriver");

            c.goto("https://somafm.com/groovesalad/songhistory.html")
                .await?;

            let html = c.find(Locator::Css("#playinc")).await?.html(true).await?;

            println!("HTML: {}", html);

            c.close().await
        };

        match res.await {
            Ok(_) => {}
            Err(e) => {
                println!("ERROR: {:#?}", e);
            }
        };

        assert!(true)
    }

    #[tokio::test]
    async fn test_03_escape_html() {
        let html_payload = r#"
<table width="100%" border="0">
<tbody>
<tr>
 <td width="15%" class="boldblue">Played At</td><td width="25%" class="boldblue">Artist</td><td width="25%" class="boldblue">Song</td><td width="30%" class="boldblue">Album</td><td width="5%" class="boldblue"></td>
 </tr>

 <tr>
 <td colspan="5"><img src="/img3/red.gif" height="1" width="100%" alt=""></td>
 </tr>

<!-- line 1 -->
<tr><td>14:21:19&nbsp; (Now) </td>
<td><a target="_blank" href="/buy/multibuy.cgi?mode=amazon&amp;title=Last%20Place%20To%20Hide&amp;artist=Welder%20%26%20Seed&amp;album=Chime&amp;album=Chime" title="Search Amazon for Welder &amp; Seed">Welder &amp; Seed</a></td><td>Last Place To Hide</td><td><a target="_blank" href="/buy/multibuy.cgi?mode=amazon&amp;title=Last%20Place%20To%20Hide&amp;album=Chime&amp;artist=Welder%20%26%20Seed" title="Search Amazon for Chime">Chime</a></td>
<td></td>
</tr>

<!-- line 2 -->
<tr><td>14:17:01</td>
<td><a target="_blank" href="/buy/multibuy.cgi?mode=amazon&amp;title=Our%20Way&amp;artist=Hazy%20J&amp;album=Cafe%20del%20Mar%2C%20Vol.%2019&amp;album=Cafe%20del%20Mar%2C%20Vol.%2019" title="Search Amazon for Hazy J">Hazy J</a></td><td>Our Way</td><td><a target="_blank" href="/buy/multibuy.cgi?mode=amazon&amp;title=Our%20Way&amp;album=Cafe%20del%20Mar%2C%20Vol.%2019&amp;artist=Hazy%20J" title="Search Amazon for Cafe del Mar, Vol. 19">Cafe del Mar, Vol. 19</a></td>
<td></td>
</tr>

<!-- line 3 -->
<tr><td>14:12:06</td>
<td><a target="_blank" href="/buy/multibuy.cgi?mode=amazon&amp;title=Cowboy%20Hero&amp;artist=Experiment&amp;album=What%20Goes%20Up&amp;album=What%20Goes%20Up" title="Search Amazon for Experiment">Experiment</a></td><td>Cowboy Hero</td><td><a target="_blank" href="/buy/multibuy.cgi?mode=amazon&amp;title=Cowboy%20Hero&amp;album=What%20Goes%20Up&amp;artist=Experiment" title="Search Amazon for What Goes Up">What Goes Up</a></td>
<td></td>
</tr>

<!-- line 4 -->
<tr><td>14:12:02</td>
<td colspan="4"><span class="dim">Break / Station ID</span></td></tr>

<!-- line 5 -->
<tr><td>14:07:32</td>
<td><a target="_blank" href="/buy/multibuy.cgi?mode=amazon&amp;title=Glamourgirl&amp;artist=Alex%20Cortiz&amp;album=Magnifico%21&amp;album=Magnifico%21" title="Search Amazon for Alex Cortiz">Alex Cortiz</a></td><td>Glamourgirl</td><td><a target="_blank" href="/buy/multibuy.cgi?mode=amazon&amp;title=Glamourgirl&amp;album=Magnifico%21&amp;artist=Alex%20Cortiz" title="Search Amazon for Magnifico!">Magnifico!</a></td>
<td></td>
</tr>

<!-- line 6 -->
<tr><td>14:02:23</td>
<td><a target="_blank" href="/buy/multibuy.cgi?mode=amazon&amp;title=Ore%20Corymb%20%5BBursting%20Rainbow%20C&amp;artist=Kiln&amp;album=Twinewheel%20%5BLost-Sides%20and%20Dusty-Gems%201994-2005%5D&amp;album=Twinewheel%20%5BLost-Sides%20and%20Dusty-Gems%201994-2005%5D" title="Search Amazon for Kiln">Kiln</a></td><td>Ore Corymb [Bursting Rainbow C</td><td><a target="_blank" href="/buy/multibuy.cgi?mode=amazon&amp;title=Ore%20Corymb%20%5BBursting%20Rainbow%20C&amp;album=Twinewheel%20%5BLost-Sides%20and%20Dusty-Gems%201994-2005%5D&amp;artist=Kiln" title="Search Amazon for Twinewheel [Lost-Sides and Dusty-Gems 1994-2005]">Twinewheel [Lost-Sides and Dusty-Gems 1994-2005]</a></td>
<td></td>
</tr>

<!-- line 7 -->
<tr><td>13:58:30</td>
<td><a target="_blank" href="https://edgeoftheuniverse2.bandcamp.com" title="More info on Edge Of The Universe">Edge Of The Universe</a></td><td>The Synthetics</td><td><a target="_blank" href="https://edgeoftheuniverse2.bandcamp.com" title="More information for Redshift">Redshift</a></td>
<td></td>
</tr>

<!-- line 8 -->
<tr><td>13:53:16</td>
<td><a target="_blank" href="/buy/multibuy.cgi?mode=amazon&amp;title=Porta%20Mia%20Via&amp;artist=Campa&amp;album=Klassik%20Lounge%20-%20Werk%204&amp;album=Klassik%20Lounge%20-%20Werk%204" title="Search Amazon for Campa">Campa</a></td><td>Porta Mia Via</td><td><a target="_blank" href="/buy/multibuy.cgi?mode=amazon&amp;title=Porta%20Mia%20Via&amp;album=Klassik%20Lounge%20-%20Werk%204&amp;artist=Campa" title="Search Amazon for Klassik Lounge - Werk 4">Klassik Lounge - Werk 4</a></td>
<td></td>
</tr>

<!-- line 9 -->
<tr><td>13:48:52</td>
<td><a target="_blank" href="/buy/multibuy.cgi?mode=amazon&amp;title=Astro%20Radio&amp;artist=Or-If-Is&amp;album=This%20Is&amp;album=This%20Is" title="Search Amazon for Or-If-Is">Or-If-Is</a></td><td>Astro Radio</td><td><a target="_blank" href="/buy/multibuy.cgi?mode=amazon&amp;title=Astro%20Radio&amp;album=This%20Is&amp;artist=Or-If-Is" title="Search Amazon for This Is">This Is</a></td>
<td></td>
</tr>

<!-- line 10 -->
<tr><td>13:44:51</td>
<td><a target="_blank" href="https://iPool.info" title="More info on Audiokonstrukte">Audiokonstrukte</a></td><td>Karmakoma</td><td><a target="_blank" href="https://iPool.info" title="More information for City Echoes">City Echoes</a></td>
<td></td>
</tr>

<!-- line 11 -->
<tr><td>13:38:14</td>
<td><a target="_blank" href="/buy/multibuy.cgi?mode=amazon&amp;title=Even%20So&amp;artist=Setsuna&amp;album=Autumn%20Time%20Vol.%203&amp;album=Autumn%20Time%20Vol.%203" title="Search Amazon for Setsuna">Setsuna</a></td><td>Even So</td><td><a target="_blank" href="/buy/multibuy.cgi?mode=amazon&amp;title=Even%20So&amp;album=Autumn%20Time%20Vol.%203&amp;artist=Setsuna" title="Search Amazon for Autumn Time Vol. 3">Autumn Time Vol. 3</a></td>
<td></td>
</tr>

<!-- line 12 -->
<tr><td>13:32:54</td>
<td><a target="_blank" href="/buy/multibuy.cgi?mode=amazon&amp;title=Forever%20Broke%20%28Fila%20Brazillia%20Remix%29&amp;artist=Yoko%20Kanno&amp;album=Cowboy%20Bebop%20Remixes&amp;album=Cowboy%20Bebop%20Remixes" title="Search Amazon for Yoko Kanno">Yoko Kanno</a></td><td>Forever Broke (Fila Brazillia Remix)</td><td><a target="_blank" href="/buy/multibuy.cgi?mode=amazon&amp;title=Forever%20Broke%20%28Fila%20Brazillia%20Remix%29&amp;album=Cowboy%20Bebop%20Remixes&amp;artist=Yoko%20Kanno" title="Search Amazon for Cowboy Bebop Remixes">Cowboy Bebop Remixes</a></td>
<td></td>
</tr>

<!-- line 13 -->
<tr><td>13:27:46</td>
<td><a target="_blank" href="/buy/multibuy.cgi?mode=amazon&amp;title=This%20Is%20All%20I%20Ask%20%28Oxygene%20Dow&amp;artist=Lounge%20Deluxe&amp;album=Cafe%20Dubai%2C%20a%20Trip%20Into%20Sunset%20Lounge%20%28The%20Best%20in%20Chill%20Out%20and%20Dessert%20Feelings%29&amp;album=Cafe%20Dubai%2C%20a%20Trip%20Into%20Sunset%20Lounge%20%28The%20Best%20in%20Chill%20Out%20and%20Dessert%20Feelings%29" title="Search Amazon for Lounge Deluxe">Lounge Deluxe</a></td><td>This Is All I Ask (Oxygene Dow</td><td><a target="_blank" href="/buy/multibuy.cgi?mode=amazon&amp;title=This%20Is%20All%20I%20Ask%20%28Oxygene%20Dow&amp;album=Cafe%20Dubai%2C%20a%20Trip%20Into%20Sunset%20Lounge%20%28The%20Best%20in%20Chill%20Out%20and%20Dessert%20Feelings%29&amp;artist=Lounge%20Deluxe" title="Search Amazon for Cafe Dubai, a Trip Into Sunset Lounge (The Best in Chill Out and Dessert Feelings)">Cafe Dubai, a Trip Into Sunset Lounge (The Best in Chill Out and Dessert Feelings)</a></td>
<td></td>
</tr>

<!-- line 14 -->
<tr><td>13:21:03</td>
<td><a target="_blank" href="/buy/multibuy.cgi?mode=amazon&amp;title=Knuddelmaus&amp;artist=Ulrich%20Schnauss&amp;album=Far%20Away%20Trains%20Passing%20By&amp;album=Far%20Away%20Trains%20Passing%20By" title="Search Amazon for Ulrich Schnauss">Ulrich Schnauss</a></td><td>Knuddelmaus</td><td><a target="_blank" href="/buy/multibuy.cgi?mode=amazon&amp;title=Knuddelmaus&amp;album=Far%20Away%20Trains%20Passing%20By&amp;artist=Ulrich%20Schnauss" title="Search Amazon for Far Away Trains Passing By">Far Away Trains Passing By</a></td>
<td></td>
</tr>

<!-- line 15 -->
<tr><td>13:15:01</td>
<td><a target="_blank" href="/buy/multibuy.cgi?mode=amazon&amp;title=Sola%20Systim&amp;artist=Underworld&amp;album=Ansum&amp;album=Ansum" title="Search Amazon for Underworld">Underworld</a></td><td>Sola Systim</td><td><a target="_blank" href="/buy/multibuy.cgi?mode=amazon&amp;title=Sola%20Systim&amp;album=Ansum&amp;artist=Underworld" title="Search Amazon for Ansum">Ansum</a></td>
<td></td>
</tr>

<!-- line 16 -->
<tr><td>13:14:58</td>
<td colspan="4"><span class="dim">Break / Station ID</span></td></tr>

<!-- line 17 -->
<tr><td>13:14:36</td>
<td colspan="4"><span class="dim">Break / Station ID</span></td></tr>

<!-- line 18 -->
<tr><td>13:08:29</td>
<td><a target="_blank" href="https://www.interchill.com/" title="Go to Liquid Stranger’s site">Liquid Stranger</a></td><td>Cryo</td><td><a target="_blank" href="https://interchill.bandcamp.com" title="More information for Cryogenic Encounters">Cryogenic Encounters</a></td>
<td></td>
</tr>

<!-- line 19 -->
<tr><td>13:05:29</td>
<td><a target="_blank" href="https://www.banabila.com" title="Go to Banabila’s site">Banabila</a></td><td>Mono Metro</td><td><a target="_blank" href="https://banabila.bandcamp.com/album/voiznoiz" title="More information for Voiz Noiz">Voiz Noiz</a></td>
<td></td>
</tr>

<!-- line 20 -->
<tr><td>13:01:23</td>
<td><a target="_blank" href="https://www.discogs.com/artist/Sofa+Lofa" title="Go to Sofa Lofa’s site">Sofa Lofa</a></td><td>Magic Shopkeeper</td><td><a target="_blank" href="https://bathysphere.co.uk" title="More information for Bathesphere recordings 7&quot;">Bathesphere recordings 7"</a></td>
<td></td>
</tr>

<tr><td colspan="5"><img src="/img3/red.gif" height="1" width="100%" alt=""></td></tr>

</tbody></table>
      "#.to_string();

        let res = html_escape::decode_html_entities(html_payload.as_str()).to_string();

        println!("HTML escaped: {}", res);

        assert!(true)
    }

    #[test]
    fn test_04_scrape_html() {
        let html_escaped_payload = r#"
<table width="100%" border="0">
<tbody><tr>
  <td width="15%" class="boldblue">Played At</td><td width="25%" class="boldblue">Artist</td><td width="25%" class="boldblue">Song</td><td width="30%" class="boldblue">Album</td><td width="5%" class="boldblue"></td></tr><tr><td colspan="5"><img src="/img3/red.gif" height="1" width="100%" alt=""></td></tr>

<!-- line 1 -->
<tr><td>14:21:19  (Now) </td>
<td><a target="_blank" href="/buy/multibuy.cgi?mode=amazon&title=Last%20Place%20To%20Hide&artist=Welder%20%26%20Seed&album=Chime&album=Chime" title="Search Amazon for Welder & Seed">Welder & Seed</a></td><td>Last Place To Hide</td><td><a target="_blank" href="/buy/multibuy.cgi?mode=amazon&title=Last%20Place%20To%20Hide&album=Chime&artist=Welder%20%26%20Seed" title="Search Amazon for Chime">Chime</a></td>
<td></td>
</tr>

<!-- line 2 -->
<tr><td>14:17:01</td>
<td><a target="_blank" href="/buy/multibuy.cgi?mode=amazon&title=Our%20Way&artist=Hazy%20J&album=Cafe%20del%20Mar%2C%20Vol.%2019&album=Cafe%20del%20Mar%2C%20Vol.%2019" title="Search Amazon for Hazy J">Hazy J</a></td><td>Our Way</td><td><a target="_blank" href="/buy/multibuy.cgi?mode=amazon&title=Our%20Way&album=Cafe%20del%20Mar%2C%20Vol.%2019&artist=Hazy%20J" title="Search Amazon for Cafe del Mar, Vol. 19">Cafe del Mar, Vol. 19</a></td>
<td></td>
</tr>

<!-- line 3 -->
<tr><td>14:12:06</td>
<td><a target="_blank" href="/buy/multibuy.cgi?mode=amazon&title=Cowboy%20Hero&artist=Experiment&album=What%20Goes%20Up&album=What%20Goes%20Up" title="Search Amazon for Experiment">Experiment</a></td><td>Cowboy Hero</td><td><a target="_blank" href="/buy/multibuy.cgi?mode=amazon&title=Cowboy%20Hero&album=What%20Goes%20Up&artist=Experiment" title="Search Amazon for What Goes Up">What Goes Up</a></td>
<td></td>
</tr>

<!-- line 4 -->
<tr><td>14:12:02</td>
<td colspan="4"><span class="dim">Break / Station ID</span></td></tr>

<!-- line 5 -->
<tr><td>14:07:32</td>
<td><a target="_blank" href="/buy/multibuy.cgi?mode=amazon&title=Glamourgirl&artist=Alex%20Cortiz&album=Magnifico%21&album=Magnifico%21" title="Search Amazon for Alex Cortiz">Alex Cortiz</a></td><td>Glamourgirl</td><td><a target="_blank" href="/buy/multibuy.cgi?mode=amazon&title=Glamourgirl&album=Magnifico%21&artist=Alex%20Cortiz" title="Search Amazon for Magnifico!">Magnifico!</a></td>
<td></td>
</tr>

<!-- line 6 -->
<tr><td>14:02:23</td>
<td><a target="_blank" href="/buy/multibuy.cgi?mode=amazon&title=Ore%20Corymb%20%5BBursting%20Rainbow%20C&artist=Kiln&album=Twinewheel%20%5BLost-Sides%20and%20Dusty-Gems%201994-2005%5D&album=Twinewheel%20%5BLost-Sides%20and%20Dusty-Gems%201994-2005%5D" title="Search Amazon for Kiln">Kiln</a></td><td>Ore Corymb [Bursting Rainbow C</td><td><a target="_blank" href="/buy/multibuy.cgi?mode=amazon&title=Ore%20Corymb%20%5BBursting%20Rainbow%20C&album=Twinewheel%20%5BLost-Sides%20and%20Dusty-Gems%201994-2005%5D&artist=Kiln" title="Search Amazon for Twinewheel [Lost-Sides and Dusty-Gems 1994-2005]">Twinewheel [Lost-Sides and Dusty-Gems 1994-2005]</a></td>
<td></td>
</tr>

<!-- line 7 -->
<tr><td>13:58:30</td>
<td><a target="_blank" href="https://edgeoftheuniverse2.bandcamp.com" title="More info on Edge Of The Universe">Edge Of The Universe</a></td><td>The Synthetics</td><td><a target="_blank" href="https://edgeoftheuniverse2.bandcamp.com" title="More information for Redshift">Redshift</a></td>
<td></td>
</tr>

<!-- line 8 -->
<tr><td>13:53:16</td>
<td><a target="_blank" href="/buy/multibuy.cgi?mode=amazon&title=Porta%20Mia%20Via&artist=Campa&album=Klassik%20Lounge%20-%20Werk%204&album=Klassik%20Lounge%20-%20Werk%204" title="Search Amazon for Campa">Campa</a></td><td>Porta Mia Via</td><td><a target="_blank" href="/buy/multibuy.cgi?mode=amazon&title=Porta%20Mia%20Via&album=Klassik%20Lounge%20-%20Werk%204&artist=Campa" title="Search Amazon for Klassik Lounge - Werk 4">Klassik Lounge - Werk 4</a></td>
<td></td>
</tr>

<!-- line 9 -->
<tr><td>13:48:52</td>
<td><a target="_blank" href="/buy/multibuy.cgi?mode=amazon&title=Astro%20Radio&artist=Or-If-Is&album=This%20Is&album=This%20Is" title="Search Amazon for Or-If-Is">Or-If-Is</a></td><td>Astro Radio</td><td><a target="_blank" href="/buy/multibuy.cgi?mode=amazon&title=Astro%20Radio&album=This%20Is&artist=Or-If-Is" title="Search Amazon for This Is">This Is</a></td>
<td></td>
</tr>

<!-- line 10 -->
<tr><td>13:44:51</td>
<td><a target="_blank" href="https://iPool.info" title="More info on Audiokonstrukte">Audiokonstrukte</a></td><td>Karmakoma</td><td><a target="_blank" href="https://iPool.info" title="More information for City Echoes">City Echoes</a></td>
<td></td>
</tr>

<!-- line 11 -->
<tr><td>13:38:14</td>
<td><a target="_blank" href="/buy/multibuy.cgi?mode=amazon&title=Even%20So&artist=Setsuna&album=Autumn%20Time%20Vol.%203&album=Autumn%20Time%20Vol.%203" title="Search Amazon for Setsuna">Setsuna</a></td><td>Even So</td><td><a target="_blank" href="/buy/multibuy.cgi?mode=amazon&title=Even%20So&album=Autumn%20Time%20Vol.%203&artist=Setsuna" title="Search Amazon for Autumn Time Vol. 3">Autumn Time Vol. 3</a></td>
<td></td>
</tr>

<!-- line 12 -->
<tr><td>13:32:54</td>
<td><a target="_blank" href="/buy/multibuy.cgi?mode=amazon&title=Forever%20Broke%20%28Fila%20Brazillia%20Remix%29&artist=Yoko%20Kanno&album=Cowboy%20Bebop%20Remixes&album=Cowboy%20Bebop%20Remixes" title="Search Amazon for Yoko Kanno">Yoko Kanno</a></td><td>Forever Broke (Fila Brazillia Remix)</td><td><a target="_blank" href="/buy/multibuy.cgi?mode=amazon&title=Forever%20Broke%20%28Fila%20Brazillia%20Remix%29&album=Cowboy%20Bebop%20Remixes&artist=Yoko%20Kanno" title="Search Amazon for Cowboy Bebop Remixes">Cowboy Bebop Remixes</a></td>
<td></td>
</tr>

<!-- line 13 -->
<tr><td>13:27:46</td>
<td><a target="_blank" href="/buy/multibuy.cgi?mode=amazon&title=This%20Is%20All%20I%20Ask%20%28Oxygene%20Dow&artist=Lounge%20Deluxe&album=Cafe%20Dubai%2C%20a%20Trip%20Into%20Sunset%20Lounge%20%28The%20Best%20in%20Chill%20Out%20and%20Dessert%20Feelings%29&album=Cafe%20Dubai%2C%20a%20Trip%20Into%20Sunset%20Lounge%20%28The%20Best%20in%20Chill%20Out%20and%20Dessert%20Feelings%29" title="Search Amazon for Lounge Deluxe">Lounge Deluxe</a></td><td>This Is All I Ask (Oxygene Dow</td><td><a target="_blank" href="/buy/multibuy.cgi?mode=amazon&title=This%20Is%20All%20I%20Ask%20%28Oxygene%20Dow&album=Cafe%20Dubai%2C%20a%20Trip%20Into%20Sunset%20Lounge%20%28The%20Best%20in%20Chill%20Out%20and%20Dessert%20Feelings%29&artist=Lounge%20Deluxe" title="Search Amazon for Cafe Dubai, a Trip Into Sunset Lounge (The Best in Chill Out and Dessert Feelings)">Cafe Dubai, a Trip Into Sunset Lounge (The Best in Chill Out and Dessert Feelings)</a></td>
<td></td>
</tr>

<!-- line 14 -->
<tr><td>13:21:03</td>
<td><a target="_blank" href="/buy/multibuy.cgi?mode=amazon&title=Knuddelmaus&artist=Ulrich%20Schnauss&album=Far%20Away%20Trains%20Passing%20By&album=Far%20Away%20Trains%20Passing%20By" title="Search Amazon for Ulrich Schnauss">Ulrich Schnauss</a></td><td>Knuddelmaus</td><td><a target="_blank" href="/buy/multibuy.cgi?mode=amazon&title=Knuddelmaus&album=Far%20Away%20Trains%20Passing%20By&artist=Ulrich%20Schnauss" title="Search Amazon for Far Away Trains Passing By">Far Away Trains Passing By</a></td>
<td></td>
</tr>

<!-- line 15 -->
<tr><td>13:15:01</td>
<td><a target="_blank" href="/buy/multibuy.cgi?mode=amazon&title=Sola%20Systim&artist=Underworld&album=Ansum&album=Ansum" title="Search Amazon for Underworld">Underworld</a></td><td>Sola Systim</td><td><a target="_blank" href="/buy/multibuy.cgi?mode=amazon&title=Sola%20Systim&album=Ansum&artist=Underworld" title="Search Amazon for Ansum">Ansum</a></td>
<td></td>
</tr>

<!-- line 16 -->
<tr><td>13:14:58</td>
<td colspan="4"><span class="dim">Break / Station ID</span></td></tr>

<!-- line 17 -->
<tr><td>13:14:36</td>
<td colspan="4"><span class="dim">Break / Station ID</span></td></tr>

<!-- line 18 -->
<tr><td>13:08:29</td>
<td><a target="_blank" href="https://www.interchill.com/" title="Go to Liquid Stranger’s site">Liquid Stranger</a></td><td>Cryo</td><td><a target="_blank" href="https://interchill.bandcamp.com" title="More information for Cryogenic Encounters">Cryogenic Encounters</a></td>
<td></td>
</tr>

<!-- line 19 -->
<tr><td>13:05:29</td>
<td><a target="_blank" href="https://www.banabila.com" title="Go to Banabila’s site">Banabila</a></td><td>Mono Metro</td><td><a target="_blank" href="https://banabila.bandcamp.com/album/voiznoiz" title="More information for Voiz Noiz">Voiz Noiz</a></td>
<td></td>
</tr>

<!-- line 20 -->
<tr><td>13:01:23</td>
<td><a target="_blank" href="https://www.discogs.com/artist/Sofa+Lofa" title="Go to Sofa Lofa’s site">Sofa Lofa</a></td><td>Magic Shopkeeper</td><td><a target="_blank" href="https://bathysphere.co.uk" title="More information for Bathesphere recordings 7"">Bathesphere recordings 7"</a></td>
<td></td>
</tr>
<tr><td colspan="5"><img src="/img3/red.gif" height="1" width="100%" alt=""></td></tr>
</tbody></table>
      "#;

        use scraper::{ElementRef, Html, Selector};

        let res = || -> Result<(), anyhow::Error> {
            let fragment = Html::parse_fragment(html_escaped_payload);

            let table_selector = Selector::parse("table")
                .map_err(|e| anyhow::anyhow!("An error occurred: {:#?}", e))?;
            let tbody_selector = Selector::parse("tbody")
                .map_err(|e| anyhow::anyhow!("An error occurred: {:#?}", e))?;
            let tr_selector = Selector::parse("tr")
                .map_err(|e| anyhow::anyhow!("An error occurred: {:#?}", e))?;
            let td_selector = Selector::parse("td")
                .map_err(|e| anyhow::anyhow!("An error occurred: {:#?}", e))?;
            let a_selector =
                Selector::parse("a").map_err(|e| anyhow::anyhow!("An error occurred: {:#?}", e))?;

            let table = fragment
                .select(&table_selector)
                .next()
                .context("An error occurred")?
                .select(&tbody_selector)
                .next()
                .context("An error occurred")?;

            'next_row: for (index, row) in table.select(&tr_selector).enumerate() {
                let tds = row.select(&td_selector).collect::<Vec<_>>();
                /*
                  Ignore first (heading) and second (separator) row
                  Ignore any row that is tds.len() < 5
                  Ignore last (separator) row -- above rule captures this rule
                */

                if index < 2 || tds.len() < 5 {
                    continue 'next_row;
                }

                let get_inner = |td: ElementRef| -> Result<String> {
                    let res = td
                        .select(&a_selector)
                        .next()
                        .context("An error occurred")?
                        .inner_html();

                    Ok(res)
                };

                println!(
                    "TDs:  \n| {} | {} | {} | {} | {} |",
                    html_escape::decode_html_entities(tds[0].inner_html().as_str()),
                    html_escape::decode_html_entities(get_inner(tds[1])?.as_str()),
                    html_escape::decode_html_entities(tds[2].inner_html().as_str()),
                    html_escape::decode_html_entities(get_inner(tds[3])?.as_str()),
                    html_escape::decode_html_entities(tds[4].inner_html().as_str()),
                );
            }

            Ok(())
        };

        match res() {
            Ok(_) => {}
            Err(e) => {
                println!("ERROR: {:#?}", e);
            }
        };

        assert!(true)
    }

    #[tokio::test]
    #[ignore]
    async fn test_05_store_scraped_result() {
        let html = try_get_playlist().await.unwrap();

        let playlist = try_scrape_html(html.as_str()).unwrap();

        println!("Playlist Vec: {:#?}", playlist);

        assert!(true)
    }
}
