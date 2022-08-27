import requests as req
from datetime import date
from fake_useragent import UserAgent
import fire
from colorama import init, Fore


init()

BASE_URL = 'https://pan.genomics.cn/ucdisk/'


def main(download_url: str, ua: str=None, export_path: str='from-bgi'):
    sess = req.Session()
    sess.headers['User-Agent'] = ua or UserAgent().chrome
    res = sess.post(
        'https://pan.genomics.cn/ucdisk/api/2.0/share/link/shareLongUrl', 
        data={'shortUrl': download_url}
    )
    resj = res.json()

    shareEventId = resj['body']['bean']['id']
    share_from = resj['body']['bean']['senderName']
    print(f'share from: {share_from} {shareEventId}')

    base_dir_res = sess.post('https://pan.genomics.cn/ucdisk/s/api/2.0/share/link/info', data={
        'shareEventId': shareEventId,
        'pageNumber': 1,
        'pageSize': 500,
        'code': ''
    })
    base_rows = base_dir_res.json()['body']['rows']

    all_files = []
    def find_all_files(base_rows: list[dict], tab=0):
        for row in base_rows:
            path = row['path']

            print('\t'*tab, path, sep='')
            if row['type'] == 'directory':
                rows_res = sess.post('https://pan.genomics.cn/ucdisk/api/2.0/share/link/rec/list/dir', data={
                    'pageNumber': 1,
                    'pageSize': 500,
                    'keyword': '',
                    'shareEventId': shareEventId,
                    'nodeId': row['id'],
                    'code': ''
                }).json()
                find_all_files(rows_res['body']['rows'], tab+1)
            else:
                all_files.append((shareEventId, path, row['id']))

    find_all_files(base_rows)

    base_dir = f'{export_path}/{date.today()}'
    with open('downlist.txt', 'w') as f:
        for share_id, path, node_id in all_files:
            f.write(f'https://pan.genomics.cn/ucdisk/api/2.0/share/link/download?shareEventId={shareEventId}&nodeId={node_id}&code=\n')
            f.write(f'\tout={path}\n')
    print('\nRun the following command to download:')
    print(Fore.GREEN + f'\taria2c -idownlist.txt -d {base_dir}')
    print(Fore.RESET)


if __name__ == '__main__':
    fire.Fire(main)
