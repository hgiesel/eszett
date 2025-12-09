CREATE TABLE IF NOT EXISTS languages (
    id SERIAL,
    CONSTRAINT languages_pkey PRIMARY KEY (id)
);

INSERT INTO languages DEFAULT VALUES; -- English
INSERT INTO languages DEFAULT VALUES; -- Spanish
INSERT INTO languages DEFAULT VALUES; -- French
INSERT INTO languages DEFAULT VALUES; -- Italian

CREATE TABLE IF NOT EXISTS parts_of_speech (
    id SERIAL,
    language_id INTEGER NOT NULL REFERENCES languages(id) ON DELETE CASCADE,
    CONSTRAINT language_pkey PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS terms (
    id SERIAL,
    language_id INTEGER NOT NULL REFERENCES languages(id) ON DELETE CASCADE,
    term TEXT NOT NULL,
    CONSTRAINT terms_pkey PRIMARY KEY (id),
    CONSTRAINT terms_language_id_term_key UNIQUE (language_id, term)
);

CREATE TABLE IF NOT EXISTS lemmas (
    id SERIAL,
    term_id INTEGER NOT NULL REFERENCES terms(id) ON DELETE CASCADE,
    CONSTRAINT lemmas_pkey PRIMARY KEY (id),
    CONSTRAINT lemmas_term_id_key UNIQUE (term_id)
);

CREATE TABLE IF NOT EXISTS lexemes (
    id SERIAL,
    part_of_speech_id INTEGER NOT NULL REFERENCES parts_of_speech(id) ON DELETE CASCADE,
    part_of_speech_position INTEGER NOT NULL,
    lemma_id INTEGER NOT NULL REFERENCES lemmas(id) ON DELETE CASCADE,
    lemma_position INTEGER,
    attributes TEXT NOT NULL,
    CONSTRAINT lexemes_pkey PRIMARY KEY (id),
    CONSTRAINT lexemes_lemma_id_part_of_speech_id_part_of_speech_position_key UNIQUE (lemma_id, part_of_speech_id, part_of_speech_position),
    CONSTRAINT lexemes_lemma_id_lemma_position_key UNIQUE (lemma_id, lemma_position)
);

CREATE TABLE IF NOT EXISTS forms (
    id SERIAL,
    term_id INTEGER NOT NULL REFERENCES terms(id) ON DELETE CASCADE,
    lexeme_id INTEGER NOT NULL REFERENCES lexemes(id) ON DELETE CASCADE,
    position INTEGER,
    CONSTRAINT forms_pkey PRIMARY KEY (id),
    CONSTRAINT forms_term_id_lexeme_id_key UNIQUE (term_id, lexeme_id),
    CONSTRAINT forms_term_id_position_key UNIQUE (term_id, position)
);

CREATE TABLE IF NOT EXISTS slots (
    id SERIAL,
    part_of_speech_id INTEGER NOT NULL REFERENCES parts_of_speech(id) ON DELETE CASCADE,
    position INTEGER,
    CONSTRAINT slots_pkey PRIMARY KEY (id),
    CONSTRAINT slots_language_id_part_of_speech_id_code_key UNIQUE (part_of_speech_id, position)
);

CREATE TABLE IF NOT EXISTS forms (
    id SERIAL,
    term_id INTEGER NOT NULL REFERENCES terms(id) ON DELETE CASCADE,
    lexeme_id INTEGER NOT NULL REFERENCES lexemes(id) ON DELETE CASCADE,
    position INTEGER,
    CONSTRAINT forms_pkey PRIMARY KEY (id),
    CONSTRAINT forms_term_id_lexeme_id_key UNIQUE (term_id, lexeme_id),
    CONSTRAINT forms_term_id_position_key UNIQUE (term_id, position)
);

CREATE TABLE IF NOT EXISTS inflections (
    id SERIAL,
    form_id INTEGER NOT NULL REFERENCES forms(id) ON DELETE CASCADE,
    slot_id INTEGER NOT NULL REFERENCES slots(id) ON DELETE CASCADE,
    attributes TEXT NOT NULL,
    position INTEGER,
    CONSTRAINT inflections_pkey PRIMARY KEY (id),
    CONSTRAINT inflections_form_id_slot_id_key UNIQUE (form_id, slot_id),
    CONSTRAINT inflections_form_id_position_key UNIQUE (form_id, position)
);