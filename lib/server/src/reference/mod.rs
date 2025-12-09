use reference::read_files;
use anyhow::Result;
use dao::connection::get_pool;
use dao::lemma::query::upsert_lemma;
use futures::future::join_all;
use project_root::get_project_root;
use dao::language::dao::LanguageDao;
use dao::term::query::upsert_term;

pub async fn initialize() -> Result<()>{
    let mut root = get_project_root()?;
    root.push("reference");
    let map = read_files(&mut root)?;
    let pool = get_pool().await;

    println!("Start upserted lemma: {:?}", map);

    let queries = map.map.into_iter().map(|(lemma, _lexemes)| async move {
        let mut tr = pool.begin().await?;

        let term_id = upsert_term(tr.as_mut(), LanguageDao::English, lemma.as_str()).await?;
        let _lemma_id = upsert_lemma(tr.as_mut(), term_id).await?;

        tr.commit().await?;
        println!("Successfully upserted lemma: {}", lemma);

        Ok(())
    }).collect::<Vec<_>>();

    join_all(queries).await.into_iter().collect()
}