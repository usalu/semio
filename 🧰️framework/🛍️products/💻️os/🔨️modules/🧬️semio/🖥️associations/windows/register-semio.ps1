# @emoji 🪟 Registers `.semio` ProgId and `application/vnd.semio` for the current user.
$progId = "Semio.Document"
$ext = ".semio"
New-Item -Path "HKCU:\Software\Classes\$ext" -Force | Out-Null
Set-ItemProperty -Path "HKCU:\Software\Classes\$ext" -Name "(default)" -Value $progId
New-Item -Path "HKCU:\Software\Classes\$progId" -Force | Out-Null
Set-ItemProperty -Path "HKCU:\Software\Classes\$progId" -Name "(default)" -Value "Semio document"
New-Item -Path "HKCU:\Software\Classes\$progId\shell\open\command" -Force | Out-Null
Set-ItemProperty -Path "HKCU:\Software\Classes\$progId\shell\open\command" -Name "(default)" -Value 'semio open "%1"'
