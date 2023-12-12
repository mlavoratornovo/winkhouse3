CREATE TABLE IF NOT EXISTS categorie_clienti(
   id_categoria INTEGER PRIMARY KEY NOT NULL,
   descrizione TEXT NOT NULL,
   id_agente_edit INTEGER,
   date_update TEXT,
   FOREIGN KEY (id_agente_edit) REFERENCES agenti (id_agente)   
);