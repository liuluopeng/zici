


package.json  scripts  中添加
```
    "cnchar-serve": "cnchar-serve 3002",
    "cnchar-prod": "cnchar-serve-prod"
```


`
npm run cnchar-serve
`




```
npm run cnchar-prod

会在项目根目录生成cnchar-data目录, 将改文件夹放置在您的http服务器中
假设可访问路径为 https://www.xxx.com/cnchar-data/
```


