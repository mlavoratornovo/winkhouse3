CREATE TABLE IF NOT EXISTS immobili(
    id_immobile INTEGER PRIMARY KEY NOT NULL,
    rif TEXT UNIQUE,
    anno_costruzione INTEGER,
    indirizzo TEXT NOT NULL,
    citta TEXT,
    provincia TEXT,
    cap TEXT,
    zona TEXT,        
    data_libero TEXT,
    descrizione TEXT,
    prezzo REAL,
    mutuo REAL,
    mutuo_descrizione TEXT,
    spese REAL,
    varie TEXT,
    visione BOOLEAN DEFAULT 0,
    storico BOOLEAN DEFAULT 0,
    affitto BOOLEAN DEFAULT 0,
    mq INTEGER,
    id_stato INTEGER,
    id_tipologia INTEGER,
    id_riscaldamento INTEGER,    
    id_classe_energetica INTEGER,
    ncivico TEXT,
    id_agente_edit INTEGER,
    date_update TEXT,
    FOREIGN KEY (id_agente_edit) REFERENCES agenti (id_agente),
    FOREIGN KEY (id_stato) REFERENCES stati_conservativi (id_stato),
    FOREIGN KEY (id_tipologia) REFERENCES tipi_immobili (id_tipologia),
    FOREIGN KEY (id_riscaldamento) REFERENCES riscaldamenti (id_riscaldamento),
    FOREIGN KEY (id_classe_energetica) REFERENCES classi_energetiche (id_classe)
);