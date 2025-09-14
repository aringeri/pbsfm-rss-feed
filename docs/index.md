---
layout: default
---
<div class="container">
{% assign feeds = site.data.feeds.pbsfm | sort %}
{% for station_hash in feeds %}
  {% assign station = station_hash[1] %}
  {% assign channel = station.rss.rss.channel %}
  <div class="row-outer">
    <div class="row">
        <div class="image-cell">
            <a href="{{channel.link}}">
                <img src="{{channel.image.url}}" alt="{{channel.image.title}} profile image" title="{{channel.image.title}} on pbsfm.org.au"/>
            </a>
        </div>
        <div class="station">
            <strong>{{ channel.title }}</strong>
            - {{ channel.category }}
            <br>Presented by: {{ channel.item | map: "author" | first }}
        </div>
        <div class="rss-cell">
            <a href="./feeds/pbsfm/{{ station_hash[0] }}/rss.xml">
                <img src="assets/images/rss.png" alt="RSS icon" title="RSS feed for {{channel.title}}"/>
            </a>
        </div>
    </div>
  </div>
{% endfor %}
</div>
