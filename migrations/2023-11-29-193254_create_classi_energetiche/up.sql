CREATE TABLE IF NOT EXISTS classi_energetiche(
   id_classe INTEGER PRIMARY KEY NOT NULL,
   nome TEXT NOT NULL,
   descrizione TEXT NOT NULL,
   id_agente_edit INTEGER,
   date_update TEXT,
   FOREIGN KEY (id_agente_edit) REFERENCES agenti (id_agente)    
);