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
                <img src="{{channel.image.url}}" alt="{{channel.image.title}}"/>
            </a>
        </div>
        <div class="station">
            <strong>{{ channel.title }}</strong>
            -
            <a href="./feeds/pbsfm/{{ station_hash[0] }}/rss.xml">
              (rss feed)
            </a>
        </div>
    </div>
  </div>
{% endfor %}
</div>
