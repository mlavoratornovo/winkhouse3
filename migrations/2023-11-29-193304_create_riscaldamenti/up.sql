CREATE TABLE IF NOT EXISTS riscaldamenti(
   id_riscaldamento INTEGER PRIMARY KEY NOT NULL,
   descrizione TEXT NOT NULL,
   id_agente_edit INTEGER,
   date_update TEXT,
   FOREIGN KEY (id_agente_edit) REFERENCES agenti (id_agente)    
);