# actix-warp-subconverter

使用actix-web框架开发的，本地web节点转换工具。WARP 转换为 v2rayN/v2rayNG、NekoBox for PC、NekoBox for Android、GUI.for.SingBox、Hiddify-next、clash-verge-rev/calsh-nyanpasu/FIClash/ClashMetaForAndroid 客户端的订阅，支持WARP优选IP。

## 一、转换示意图

<img src="images\转换示意图.png" />

## 二、使用

先将 Cloudflare WARP 对应的 WireGuard 配置参数写入 config.yaml 文件中。格式如下：

```yaml
warp_parameters:
  - private_key: +Cgu25E1zo9PkW5fC299zgbGVGKJamWgF6/iqQdoUW0=
    public_key: bmXOC+F1FxEMF9dyiK2H5/1SUtzH0JuVo51h2wPfgyo=
    reserved: 
    v4: 172.16.0.2
    v6: 2606:4700:110:805e:1441:a533:975b:8a39
  - private_key: GKaNRx+KVRL3F1sguZHO8wh70yUprNsPjmUapCGUsGk=
    public_key: bmXOC+F1FxEMF9dyiK2H5/1SUtzH0JuVo51h2wPfgyo=
    reserved: [ 121, 102, 72 ]
    v4: 172.16.0.2
    v6: 2606:4700:110:88f9:54b8:120e:d01d:c77e
  - private_key: qEqlXOEDcFt803y8Gs/fo8DuZJpZpWV/FSh1oKReFXI=
    public_key: bmXOC+F1FxEMF9dyiK2H5/1SUtzH0JuVo51h2wPfgyo=
    reserved: [ 18, 15, 251 ]
    v4: 172.16.0.2
    v6: 2606:4700:110:890f:f926:98fe:7e61:d0e7
  - private_key: +HfkMSyh7obEkX4J8Qa7Xk77CLVn45AW4CdBbnFNaGc=
    public_key: bmXOC+F1FxEMF9dyiK2H5/1SUtzH0JuVo51h2wPfgyo=
    reserved: [ 92, 242, 140 ]
    v4: 172.16.0.2
    v6: 2606:4700:110:83e8:84f7:8c64:70b4:6709
  - private_key: cA8htoCSuLJbax8I6HewsDTwTbuWt5DjEItcGvLGREw=
    public_key: bmXOC+F1FxEMF9dyiK2H5/1SUtzH0JuVo51h2wPfgyo=
    reserved: [ 50, 15, 234 ]
    v4: 172.16.0.2
    v6: 2606:4700:110:8c0b:359c:ee61:a221:d261
```