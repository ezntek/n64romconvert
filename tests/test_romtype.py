from . import *

def test_str_romtype_conversion():
    string = "z64"
    rom_type = RomType.from_str(string)

    assert rom_type is RomType.BigEndian
