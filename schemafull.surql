-- table nodes
DEFINE TABLE nodes SCHEMALESS PERMISSIONS NONE;
-- fields table nodes
DEFINE FIELD nodeType ON TABLE nodes TYPE option<string>
  ASSERT IF $value THEN $value INSIDE ['dir', 'file', 'symlink'] ELSE true END;
DEFINE FIELD name ON TABLE nodes TYPE string ASSERT $value != NONE;
DEFINE FIELD path ON TABLE nodes TYPE string ASSERT $value != NONE;
DEFINE FIELD canonical_path ON TABLE nodes TYPE option<string>;
DEFINE FIELD size ON TABLE nodes TYPE number ASSERT $value != NONE;
DEFINE FIELD created ON TABLE nodes TYPE datetime ASSERT $value != NONE;
DEFINE FIELD modified ON TABLE nodes TYPE datetime ASSERT $value != NONE;
DEFINE FIELD accessed ON TABLE nodes TYPE datetime ASSERT $value != NONE;
DEFINE FIELD sha256 ON TABLE nodes TYPE option<string>;
DEFINE FIELD parent ON TABLE nodes TYPE option<record>;
DEFINE FIELD notes ON TABLE nodes TYPE option<string>;
DEFINE FIELD published ON TABLE nodes TYPE bool DEFAULT false;
DEFINE FIELD created_at ON TABLE nodes TYPE datetime DEFAULT time::now() ASSERT $value != NONE;
