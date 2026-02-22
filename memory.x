MEMORY {
    BOOT2 : ORIGIN = 0x10000000, LENGTH = 0x100
    /* 2048K - 256K (storage) - 0x100 (boot2) = 1791.75K available for code */
    FLASH : ORIGIN = 0x10000100, LENGTH = 1842944 
    
    /* New storage region at the end of flash */
    STORAGE : ORIGIN = 0x101C0000, LENGTH = 256K

    RAM   : ORIGIN = 0x20000000, LENGTH = 264K
}