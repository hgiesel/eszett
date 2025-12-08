INSERT INTO terms (language_id, term)
VALUES (
    :languageId,
    :term
)
ON CONFLICT (language_id, term)
DO NOTHING
RETURNING id;