# BGI-Cloud-Disk-Batch-Downloader
> pan.genomics.cn Downloader

BGI Cloud Disk does not support batch downloading of folders, so I wrote this tool. 

## Useage

> Sample url from random search on Github

```bash
$ python src/main.py https://pan.genomics.cn/ucdisk/s/YVb63y
share from: 余佳临(Jialin Yu) share_201896153254859_9a4c734f7d3545168f5ed8eddd9e2703

/00-Dr.Tom交互式报告系统解读视频/Dr.Tom系统讲解---RNA seq
        /00-Dr.Tom交互式报告系统解读视频/Dr.Tom系统讲解---RNA seq/00.摘要
                /00-Dr.Tom交互式报告系统解读视频/Dr.Tom系统讲解---RNA seq/00.摘要/00.交互式报告-摘要.mp4
        /00-Dr.Tom交互式报告系统解读视频/Dr.Tom系统讲解---RNA seq/01.基础分析
                /00-Dr.Tom交互式报告系统解读视频/Dr.Tom系统讲解---RNA seq/01.基础分析/01.数据过滤.mp4
                /00-Dr.Tom交互式报告系统解读视频/Dr.Tom系统讲解---RNA seq/01.基础分析/02.参考基因组比对.mp4
                /00-Dr.Tom交互式报告系统解读视频/Dr.Tom系统讲解---RNA seq/01.基础分析/03.参考基因比对.mp4
                /00-Dr.Tom交互式报告系统解读视频/Dr.Tom系统讲解---RNA seq/01.基础分析/04.随机性.mp4
                /00-Dr.Tom交互式报告系统解读视频/Dr.Tom系统讲解---RNA seq/01.基础分析/05.覆盖度.mp4
                /00-Dr.Tom交互式报告系统解读视频/Dr.Tom系统讲解---RNA seq/01.基础分析/06.测序饱和度.mp4
        /00-Dr.Tom交互式报告系统解读视频/Dr.Tom系统讲解---RNA seq/02.基因信息
                /00-Dr.Tom交互式报告系统解读视频/Dr.Tom系统讲解---RNA seq/02.基因信息/已知基因.mp4
        /00-Dr.Tom交互式报告系统解读视频/Dr.Tom系统讲解---RNA seq/03.基因注释
                /00-Dr.Tom交互式报告系统解读视频/Dr.Tom系统讲解---RNA seq/03.基因注释/01.基因注释-总览.mp4
                /00-Dr.Tom交互式报告系统解读视频/Dr.Tom系统讲解---RNA seq/03.基因注释/02.TF.mp4
        /00-Dr.Tom交互式报告系统解读视频/Dr.Tom系统讲解---RNA seq/04.基因定量
                /00-Dr.Tom交互式报告系统解读视频/Dr.Tom系统讲解---RNA seq/04.基因定量/01. GeneExp.mp4
                /00-Dr.Tom交互式报告系统解读视频/Dr.Tom系统讲解---RNA seq/04.基因定量/02.样本相关性.mp4
                /00-Dr.Tom交互式报告系统解读视频/Dr.Tom系统讲解---RNA seq/04.基因定量/03.PCA.mp4
                /00-Dr.Tom交互式报告系统解读视频/Dr.Tom系统讲解---RNA seq/04.基因定量/04.表达量分布.mp4
                /00-Dr.Tom交互式报告系统解读视频/Dr.Tom系统讲解---RNA seq/04.基因定量/05.基因表达韦恩.mp4
                /00-Dr.Tom交互式报告系统解读视频/Dr.Tom系统讲解---RNA seq/04.基因定量/06.时间序列.mp4
                /00-Dr.Tom交互式报告系统解读视频/Dr.Tom系统讲解---RNA seq/04.基因定量/07.WGCNA.mp4
        /00-Dr.Tom交互式报告系统解读视频/Dr.Tom系统讲解---RNA seq/05.差异表达基因
                /00-Dr.Tom交互式报告系统解读视频/Dr.Tom系统讲解---RNA seq/05.差异表达基因/01.组间差异.mp4
                /00-Dr.Tom交互式报告系统解读视频/Dr.Tom系统讲解---RNA seq/05.差异表达基因/02.样本间差异分析.mp4
                /00-Dr.Tom交互式报告系统解读视频/Dr.Tom系统讲解---RNA seq/05.差异表达基因/03.差异基因Venn图.mp4
                /00-Dr.Tom交互式报告系统解读视频/Dr.Tom系统讲解---RNA seq/05.差异表达基因/04.差异基因聚类.mp4
                /00-Dr.Tom交互式报告系统解读视频/Dr.Tom系统讲解---RNA seq/05.差异表达基因/05.GO分析.mp4
                /00-Dr.Tom交互式报告系统解读视频/Dr.Tom系统讲解---RNA seq/05.差异表达基因/06.KEGG Pathway分析.mp4
        /00-Dr.Tom交互式报告系统解读视频/Dr.Tom系统讲解---RNA seq/001-页面概览.mp4
        /00-Dr.Tom交互式报告系统解读视频/Dr.Tom系统讲解---RNA seq/002.图形区交互.mp4
        /00-Dr.Tom交互式报告系统解读视频/Dr.Tom系统讲解---RNA seq/003.表格扩展和交互.mp4
        /00-Dr.Tom交互式报告系统解读视频/Dr.Tom系统讲解---RNA seq/004.分析工具交互.mp4

Run the following command to download:
        aria2c -idownlist.txt -d from-bgi/2022-08-18
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
