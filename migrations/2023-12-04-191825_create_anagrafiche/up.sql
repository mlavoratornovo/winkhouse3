CREATE TABLE IF NOT EXISTS anagrafiche(
    id_anagrafica INTEGER PRIMARY KEY NOT NULL,
    cognome TEXT NOT NULL,
    nome TEXT,
    ditta BOOLEAN,
    indirizzo TEXT,
    provincia TEXT,
    cap TEXT,
    citta TEXT,    
    commento TEXT,
    storico BOOLEAN,    
    id_categoria INTEGER,
    codicefiscale TEXT,
    piva TEXT,
    ncivico TEXT,
    id_agente_edit INTEGER,
    date_update TEXT,
    FOREIGN KEY (id_agente_edit) REFERENCES agenti (id_agente),
    FOREIGN KEY(id_categoria) REFERENCES categorie_clienti (id_categoria)
);