# BGI-Cloud-Disk-Batch-Downloader
> pan.genomics.cn Downloader

BGI Cloud Disk does not support batch downloading of folders, so I wrote this tool. 

## Useage

```bash
$ python src/main.py https://pan.genomics.cn/ucdisk/s/******
share from: ****** share_2022810**********_**********************

/34mb_addshanghai
        /34mb_addshanghai/********************.gem.gz
        /34mb_addshanghai/md5sum.txt
        /34mb_addshanghai/sample.txt
        /34mb_addshanghai/********************BL_D3.gem.gz
        /34mb_addshanghai/********************BL_B6.gem.gz
        /34mb_addshanghai/********************BL_B1.gem.gz
        /34mb_addshanghai/********************BL_A4.gem.gz
        /34mb_addshanghai/********************BL_F5.gem.gz
        /34mb_addshanghai/********************BL_B1.gem.gz
        /34mb_addshanghai/********************BL_A5.gem.gz
        /34mb_addshanghai/********************TL_F2.gem.gz
        /34mb_addshanghai/********************TL_E4.gem.gz
        /34mb_addshanghai/********************TL_C1.gem.gz
        /34mb_addshanghai/********************TL_A6.gem.gz
        /34mb_addshanghai/********************TR_F3.gem.gz
        /34mb_addshanghai/********************TR_C3.gem.gz
        /34mb_addshanghai/********************TR_A5.gem.gz
        /34mb_addshanghai/********************TR_A4.gem.gz
        /34mb_addshanghai/********************TL_D6.gem.gz
        /34mb_addshanghai/********************TL_A2.gem.gz
        /34mb_addshanghai/********************TL_E2.gem.gz
        /34mb_addshanghai/********************TL_B6.gem.gz
        /34mb_addshanghai/********************TL_F6.gem.gz
        /34mb_addshanghai/********************TL_B6.gem.gz
        /34mb_addshanghai/********************TL_B4.gem.gz
        /34mb_addshanghai/********************BR_A2.gem.gz

Run the following command to download:
        aria2c -idownlist.txt -d from-bgi/2022-08-16

```

help: 
```bash
NAME
    main.py

SYNOPSIS
    main.py DOWNLOAD_URL <flags>

POSITIONAL ARGUMENTS
    DOWNLOAD_URL
        Type: str

FLAGS
    --ua=UA
        Type: Optional[str]
        Default: None
    --export_path=EXPORT_PATH
        Type: str
        Default: 'from-bgi'

NOTES
    You can also use flags syntax for POSITIONAL ARGUMENTS
```

## Warning

Don't stress the server too much.
