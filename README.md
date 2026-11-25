# Prescrire

**Rédaction de CCTP et de DPGF pour les architectes maîtres d'œuvre.** Application de bureau pour macOS et Windows, gratuite et libre.

Prescrire réunit en un seul outil la bibliothèque de prescriptions, le CCTP et la DPGF : les deux documents viennent de la même source et restent toujours cohérents.

![Interface principale de Prescrire : arbre des lots, fiche prestation et CCTP](docs/captures/interface.jpg)

## Fonctions

**Rédaction**

- **Lots, sous-lots et prestations** organisés par glisser-déposer, numérotation automatique.
- **Fiche prestation** reliée à une bibliothèque d'articles, avec autocomplétion. Toute nouvelle prestation y est ajoutée.
- **Descriptif détaillé** en clauses (préalables, exécution, limites de prestation), imprimées en alinéas a), b), c)… ; pour chaque lot ou sous-lot, les DTU de référence et des clauses générales imprimées une seule fois.
- **Notes internes** datées et signées, jamais imprimées.
- **État de rédaction** de chaque prestation (en rédaction, à relire, validée, à réviser), visible en couleur dans l'arborescence et reporté sur les lots.
- **Lieux** par niveau, avec surface, périmètre et hauteur. Les quantités acceptent des formules (`P*H-4` pour les murs d'une pièce, ouvertures déduites) et se ventilent par pièce.
- **Bibliothèque partagée** par l'agence, avec repérage et fusion des doublons ; partage d'une fiche ou de la rédaction d'un lot entier.

**Documents**

- **CCTP et DPGF** affichés en direct, par lots ou par lieux, avec un récapitulatif HT / TVA / TTC et des PSE chiffrées à part.
- **Exports** : CCTP en Word ou en PDF ; DPGF en Excel, version maître d'ouvrage avec prix ou version entreprises avec prix unitaires vides et totaux par formules.
- **Modèle Word de l'agence** : votre page de garde, vos en-têtes et vos styles, avec des champs `{projet}`, `{moa}`, `{phase}`… remplis automatiquement.
- **Check-list** avant publication et **indices de DCE** : figer un indice et voir ce qui a changé depuis.
- **Aide intégrée** : manuel, questions fréquentes et raccourcis clavier.

**Modules complémentaires** (réservés à une future version commerciale, hors de ce dépôt)

- **Analyse des offres** : import des DPGF remplies par les entreprises, comparaison avec l'estimation, moins-disant et écarts signalés ; export en Excel avec une feuille par lot.
- **Planning** : durées en jours ouvrés, enchaînements entre lots, jours fériés et congés de chantier sautés ; export Excel et PDF A3.
- **Suivi de chantier** : marchés figés par lot, travaux supplémentaires (TS) attribués à une entreprise, plus et moins-values calculées d'après la DPGF, avancement, alerte de dépassement ; fiche TS en PDF.

## Aperçu

Les captures du planning, de l'analyse des offres et du suivi de chantier montrent les modules complémentaires.

| | |
|---|---|
| ![Planning avec enchaînements entre les lots](docs/captures/planning.jpg) | ![Analyse des offres de cinq entreprises](docs/captures/offres.jpg) |
| **Planning** : durées, enchaînements et dates calculées | **Analyse des offres** : moins-disant en vert, écarts à l'estimation |
| ![Suivi de chantier d'un lot avec un TS accepté](docs/captures/chantier-lot.jpg) | ![Interface en mode sombre avec la DPGF](docs/captures/interface-sombre.jpg) |
| **Suivi de chantier** : TS, alerte de dépassement, avancement | **Mode sombre**, ici avec le récapitulatif de la DPGF |

![Fenêtre d'aide : manuel d'utilisation, questions fréquentes et raccourcis](docs/captures/aide.jpg)

Les projets s'enregistrent en fichiers `.caz`. L'application fonctionne sans connexion internet.

## Télécharger

Dans la page [**Releases**](../../releases/latest) :

- **Mac** : le fichier `.dmg` (Mac Intel et Apple Silicon, macOS 11 ou plus récent).
- **Windows** : le fichier se terminant par `-setup.exe` (Windows 10 et 11, sans droits administrateur).

L'application n'est pas signée par Apple ni par Microsoft. Au premier lancement :

- **Mac** : macOS refuse de l'ouvrir. Allez dans *Réglages Système › Confidentialité et sécurité* et cliquez sur *Ouvrir quand même*.
- **Windows** : sur l'écran « Windows a protégé votre ordinateur », cliquez sur *Informations complémentaires*, puis *Exécuter quand même*.

La bibliothèque fournie au départ est un exemple : ses textes et ses prix sont indicatifs et doivent être remplacés par les vôtres (*Bibliothèque › Importer depuis Excel*).

## Bibliothèque partagée

Dans *Bibliothèque › Changer de dossier…*, choisissez un dossier du serveur de l'agence. La bibliothèque y est enregistrée sous `bibliotheque-prescrire.json`. Chaque article garde la date et l'auteur de sa dernière modification ; quand deux postes écrivent en même temps, les modifications sont fusionnées article par article.

Les projets `.caz` peuvent aussi être rangés sur le serveur : un fichier caché `.nom.caz.verrou` prévient les autres postes qu'un projet est déjà ouvert.

## Organisation du code

- `src/` : l'interface (HTML et JavaScript), avec ses polices et ses modules Excel (ExcelJS) et Word (JSZip) intégrés.
- `src-tauri/` : la coquille de bureau en Rust ([Tauri 2](https://tauri.app)) : fenêtre, fichiers, dialogues, association des fichiers `.caz`.
- `.github/workflows/fabriquer.yml` : fabrique les installateurs Mac et Windows à chaque envoi sur `main` et les publie dans *Releases*, sous le numéro de version de `src-tauri/tauri.conf.json`.

Pour publier une nouvelle version, augmentez le numéro dans `src-tauri/tauri.conf.json`, `src-tauri/Cargo.toml` et `package.json`, puis envoyez les modifications sur `main`.

## Licence

Les modules de base de Prescrire sont un logiciel libre, distribué sous licence [GNU GPL version 3](LICENSE) ou toute version ultérieure. Vous pouvez les utiliser, les étudier, les modifier et les redistribuer ; toute version modifiée diffusée doit rester sous la même licence, avec son code source.

Les modules Analyse des offres, Planning et Suivi de chantier deviennent des modules complémentaires, proposés à l'avenir sous licence commerciale, en dehors de ce dépôt ; leurs versions déjà publiées ici restent sous GPL. Les modules de base n'ont pas vocation à être commercialisés pour le moment. Détails dans [LICENSING.md](LICENSING.md).

Ce logiciel est fourni sans aucune garantie. Les prescriptions et les prix produits restent sous la responsabilité de leur auteur.
