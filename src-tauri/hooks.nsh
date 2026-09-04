!macro NSIS_HOOK_POSTINSTALL
  CopyFiles /SILENT "$INSTDIR\resources\WebView2Loader.dll" "$INSTDIR\WebView2Loader.dll"
!macroend

!macro NSIS_HOOK_POSTUNINSTALL
  Delete "$INSTDIR\WebView2Loader.dll"
!macroend
