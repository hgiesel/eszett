CREATE TABLE IF NOT EXISTS terms (
    id SERIAL,
    language_id INTEGER NOT NULL REFERENCES languages(id) ON DELETE CASCADE,
    term TEXT NOT NULL,
    CONSTRAINT terms_pkey PRIMARY KEY (id),
    CONSTRAINT terms_language_id_term_key UNIQUE(language_id, term)
);

CREATE TABLE IF NOT EXISTS languages (
    id SERIAL,
    CONSTRAINT languages_pkey PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS lemmas (
    id SERIAL,
    term_id INTEGER NOT NULL REFERENCES terms(id) ON DELETE CASCADE,
    CONSTRAINT lemmas_pkey PRIMARY KEY (id),
    CONSTRAINT lemmas_term_id_key UNIQUE(term_id)
);

CREATE TABLE IF NOT EXISTS parts_of_speech (
    id SERIAL,
    language_id INTEGER NOT NULL REFERENCES languages(id) ON DELETE CASCADE,
    CONSTRAINT parts_of_speech_pkey PRIMARY KEY (id)
);
