require 'net/http'
require 'active_support'
require 'active_support/isolated_execution_state'
require 'active_support/core_ext/hash'

module Jekyll_Xml_Data
  class Generator < Jekyll::Generator
    safe true
    priority :highest

    def generate(site)
      data_source = (site.config['xml_data_source'] || '_data')

      Dir.glob("#{data_source}/**/*.xml").each do |f|
        station = File.basename(f, ".xml")
        segments = f.split('/')
        segments.pop() # remove xml file name

        File.open(f,"r") do |file|
            xml_hash = Hash.from_xml(file.read())

            cur_hash = site.data
            segments.each do |segment|
                if !cur_hash.key?(segment)
                    cur_hash[segment] = Hash.new
                end
                cur_hash = cur_hash[segment]
            end
            cur_hash[station] = xml_hash
        end
      end
    end
  end
end
