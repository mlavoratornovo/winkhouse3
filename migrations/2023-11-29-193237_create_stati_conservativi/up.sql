CREATE TABLE IF NOT EXISTS stati_conservativi(
   id_stato INTEGER PRIMARY KEY NOT NULL,
   descrizione TEXT NOT NULL,
   id_agente_edit INTEGER,
   date_update TEXT,
   FOREIGN KEY (id_agente_edit) REFERENCES agenti (id_agente) 
);