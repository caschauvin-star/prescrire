; Icône propre aux fichiers de projet .caz (la main « OK » vue de près)
!macro NSIS_HOOK_POSTINSTALL
  WriteRegStr SHCTX "Software\Classes\Projet Prescrire\DefaultIcon" "" "$INSTDIR\caz.ico,0"
  System::Call 'shell32::SHChangeNotify(i 0x08000000, i 0, p 0, p 0)'
!macroend
