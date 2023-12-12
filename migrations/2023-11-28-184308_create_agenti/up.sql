CREATE TABLE IF NOT EXISTS agenti(
   id_agente INTEGER PRIMARY KEY NOT NULL,
   nome TEXT NOT NULL,
   cognome TEXT NOT NULL,
   indirizzo TEXT,
   provincia TEXT,
   cap TEXT,
   citta TEXT,
   username TEXT NOT NULL,
   pass_word TEXT NOT NULL UNIQUE,
   id_agente_edit INTEGER,
   date_update TEXT,
   is_admin BOOLEAN DEFAULT 0,
   FOREIGN KEY (id_agente_edit) REFERENCES agenti (id_agente) 
);