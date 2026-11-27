; Icône propre aux fichiers de bibliothèque .cazb (la main « OK » tenant un stylo)
!macro NSIS_HOOK_POSTINSTALL
  WriteRegStr SHCTX "Software\Classes\Prescrire.Bibliotheque\DefaultIcon" "" "$INSTDIR\cazb.ico,0"
  System::Call 'shell32::SHChangeNotify(i 0x08000000, i 0, p 0, p 0)'
!macroend
