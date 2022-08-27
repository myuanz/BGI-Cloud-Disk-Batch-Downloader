# BGI-Cloud-Disk-Batch-Downloader
> pan.genomics.cn Downloader

BGI Cloud Disk does not support batch downloading of folders, so I wrote this tool. 

## Install 

Download and extract from release. 

Or you can clone the repository and run `cargo build --release`

## Useage

> Sample url from random search on Github

```bash
$ ./bgi_cloud_disk_batch_downloader --url https://pan.genomics.cn/ucdisk/s/YVb63y
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
download to ./download-from-bgi/20220827

08/27 16:46:14 [NOTICE] Downloading 27 item(s)
08/27 16:46:14 [NOTICE] Allocating disk space. Use --file-allocation=none to disable it. See --file-allocation option in man page for more details.
[DL:0B][#c34c8f 0B/133MiB(0%)][#3f247a 0B/112MiB(0%)][#59d2d9 0B/93MiB(0%)][#dc3585 0B/90MiB(0%)][#5af78d 0B/81MiB(0%)] [FileAlloc:#3f247a 99MiB/112MiB(88%)](+4)^C     
08/27 16:46:16 [NOTICE] Shutdown sequence commencing... Press Ctrl-C again for emergency shutdown.
myuan@DESKTOP-ONKMH7V /m/c/U/m/p/B/t/x/release (master) [SIGINT]>
08/27 16:46:16 [NOTICE] Download GID#c34c8f7f50d4f76f not complete: ./download-from-bgi/20220827//00-Dr.Tom交互式报告系统解读视频/Dr.Tom系统讲解---RNA seq/00.摘要/00.交互式报告-摘要.mp4

08/27 16:46:16 [NOTICE] Download GID#3f247a792832194e not complete: ./download-from-bgi/20220827//00-Dr.Tom交互式报告系统解读视频/Dr.Tom系统讲解---RNA seq/01.基础分析/01.数据过滤.mp4

Download Results:
gid   |stat|avg speed  |path/URI
======+====+===========+=======================================================
c34c8f|INPR|       0B/s|./download-from-bgi/20220827//00-Dr.Tom交互式报告系统解读视频/Dr.Tom系统讲解---RNA seq/00.摘要/00.交互式报告-摘要.mp4
3f247a|INPR|   4.2MiB/s|./download-from-bgi/20220827//00-Dr.Tom交互式报告系统解读视频/Dr.Tom系统讲解---RNA seq/01.基础分析/01.数据过滤.mp4
```

help: 
```bash
bgi_cloud_disk_batch_downloader 0.1
myuan
华大基因云盘批量下载器

USAGE:
    bgi_cloud_disk_batch_downloader [OPTIONS] --url <URL>

OPTIONS:
    -e, --export-root <EXPORT_ROOT>    export to [default: ./download-from-bgi]
    -h, --help                         Print help information
    -u, --url <URL>                    shared url from BGI
    -V, --version                      Print version information
```

## Warning

Don't stress the server too much.
