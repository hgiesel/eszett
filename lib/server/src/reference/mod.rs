use reference::read_files;
use anyhow::Result;
use dao::connection::get_pool;
use dao::lemma::query::upsert_lemma;
use dao::term::query::upsert_term;
use futures::future::join_all;
use project_root::get_project_root;

pub async fn initialize() -> Result<()>{
    let pool = get_pool().await;
    let mut root = get_project_root()?;
    root.push("reference");
    let map = read_files(&mut root)?;

    println!("Start upserted lemma: {:?}", map);

    let x = map.map.into_iter().map(|(lemma, _lexemes)| async move {
        let term_id = upsert_term(pool, 1, lemma.as_str()).await?;
        let _lemma_id = upsert_lemma(pool, term_id).await?;
        println!("Successfully upserted lemma: {}", lemma);

        Ok(())
    }).collect::<Vec<_>>();

    join_all(x).await.into_iter().collect()
}