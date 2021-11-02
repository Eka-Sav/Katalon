<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_All products                           _87dcd7</name>
   <tag></tag>
   <elementGuidId>7dca45fd-53ae-44ce-8a61-739833d626d5</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>nav.navbar.navbar-inverse-light.navbar-static-top > div.container</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>(.//*[normalize-space(text()) and normalize-space(.)='Contact Us'])[1]/following::div[1]</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>container</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
        
    
        
            All products 
        


    
    
                
                    
                        
                            
                                Carriage Bolts
                            
                        
                    
                        
                            
                                Laser Printers
                            
                        
                    
                        
                            
                                PC
                            
                        
                    
                    
                        
                            
                                Flange Bolts
                            
                        
                    
                        
                            
                                Inkjet Printers
                            
                        
                    
                
            

        
        
            &lt;div class=&quot;container-fluid container-drain container-bordered&quot; ng-init='links = [{&quot;type&quot;:null,&quot;title&quot;:&quot;Carriage Bolts&quot;,&quot;url&quot;:&quot;~/bolts/carriage-bolts&quot;,&quot;priority&quot;:40,&quot;associatedObjectId&quot;:null,&quot;associatedObjectType&quot;:null,&quot;indexKey&quot;:&quot;23559533be45423581da213bcc4b35bb&quot;,&quot;id&quot;:&quot;23559533be45423581da213bcc4b35bb&quot;},{&quot;type&quot;:null,&quot;title&quot;:&quot;Flange Bolts&quot;,&quot;url&quot;:&quot;~/bolts/bolts/flange-bolts&quot;,&quot;priority&quot;:30,&quot;associatedObjectId&quot;:null,&quot;associatedObjectType&quot;:null,&quot;indexKey&quot;:&quot;07f9b26722054bc8a482240b9cdf5328&quot;,&quot;id&quot;:&quot;07f9b26722054bc8a482240b9cdf5328&quot;},{&quot;type&quot;:null,&quot;title&quot;:&quot;Laser Printers&quot;,&quot;url&quot;:&quot;~/printers/multifunction-printers&quot;,&quot;priority&quot;:20,&quot;associatedObjectId&quot;:null,&quot;associatedObjectType&quot;:null,&quot;indexKey&quot;:&quot;20eb4f00f69043e49c27955bb78fb587&quot;,&quot;id&quot;:&quot;20eb4f00f69043e49c27955bb78fb587&quot;},{&quot;type&quot;:null,&quot;title&quot;:&quot;Inkjet Printers&quot;,&quot;url&quot;:&quot;~/printers/all-in-one&quot;,&quot;priority&quot;:10,&quot;associatedObjectId&quot;:null,&quot;associatedObjectType&quot;:null,&quot;indexKey&quot;:&quot;19736144ee894797a795b5b36e4ad21c&quot;,&quot;id&quot;:&quot;19736144ee894797a795b5b36e4ad21c&quot;},{&quot;type&quot;:null,&quot;title&quot;:&quot;PC&quot;,&quot;url&quot;:&quot;pc/desktops&quot;,&quot;priority&quot;:0,&quot;associatedObjectId&quot;:null,&quot;associatedObjectType&quot;:null,&quot;indexKey&quot;:&quot;1f046b65-4e13-48d2-8150-80b79d1573ce&quot;,&quot;id&quot;:&quot;1f046b65-4e13-48d2-8150-80b79d1573ce&quot;}]'>
                &lt;div class=&quot;row&quot;>
                    &lt;div class=&quot;col-md-6&quot; ng-repeat=&quot;link in links&quot; ng-if=&quot;$even&quot;>
                        &lt;ul class=&quot;nav nav-sm nav-pills nav-stacked text-nowrap&quot;>
                            &lt;li>
                                &lt;a ng-href=&quot;{{ link.url}}&quot; ng-bind=&quot;link.title&quot;>
                                &lt;/a>
                            &lt;/li>
                        &lt;/ul>
                    &lt;/div>
                    &lt;div class=&quot;col-md-6&quot; ng-repeat=&quot;link in links&quot; ng-if=&quot;!$even&quot;>
                        &lt;ul class=&quot;nav nav-sm nav-pills nav-stacked text-nowrap&quot;>
                            &lt;li>
                                &lt;a ng-href=&quot;{{ link.url}}&quot; ng-bind=&quot;link.title&quot;>
                                &lt;/a>
                            &lt;/li>
                        &lt;/ul>
                    &lt;/div>
                &lt;/div>
            &lt;/div>
               
    


        
            
                
                
    
        Bulk Order Pad 
    
    
        &lt;div class=&quot;panel panel-default&quot; ng-init=&quot;state = 'manual'&quot;>
    
    &lt;div class=&quot;border-bottom-light&quot;>
        &lt;ul class=&quot;nav nav-pills nav-pills-transparent&quot;>
            &lt;li ng-class=&quot;{active: state == 'manual'}&quot;>
                &lt;a href=&quot;&quot; ng-click=&quot;state = 'manual'&quot;>Manually&lt;/a>
            &lt;/li>
            &lt;li ng-class=&quot;{active: state == 'csv'}&quot;>
                &lt;a href=&quot;&quot; ng-click=&quot;state = 'csv'&quot;>Copy &amp; paste&lt;/a>
            &lt;/li>
        &lt;/ul>
    &lt;/div>
    &lt;div class=&quot;panel-body&quot; ng-controller=&quot;productWithVariationsController&quot;>
            &lt;form action=&quot;/B2B-store/bulkorder/addfielditems&quot; method=&quot;POST&quot; ng-show=&quot;state == 'manual'&quot;>
                
                
                
                
                &lt;div class=&quot;container-fluid&quot;>
                    &lt;div class=&quot;row&quot;>
    
    
        &lt;div class=&quot;col-md-7&quot;>
            &lt;label class=&quot;form-label&quot;>Product SKU&lt;/label>
        &lt;/div>
        &lt;div class=&quot;col-md-5&quot;>
            &lt;label class=&quot;form-label&quot;>Quantity&lt;/label>
        &lt;/div>
    
&lt;/div>


    &lt;div class=&quot;form-group row&quot;>
        
        
            
            &lt;div class=&quot;col-md-7&quot;>
                &lt;input class=&quot;form-control&quot; name=&quot;[0].Sku&quot; type=&quot;text&quot; value=&quot;&quot; />
            &lt;/div>
            &lt;div class=&quot;col-md-5&quot;>
                &lt;input class=&quot;form-control&quot; name=&quot;[0].Quantity&quot; min=&quot;1&quot; type=&quot;number&quot; value=&quot;1&quot; />
            &lt;/div>
        
    &lt;/div>

    &lt;div class=&quot;form-group row&quot;>
        
        
            
            &lt;div class=&quot;col-md-7&quot;>
                &lt;input class=&quot;form-control&quot; name=&quot;[1].Sku&quot; type=&quot;text&quot; value=&quot;&quot; />
            &lt;/div>
            &lt;div class=&quot;col-md-5&quot;>
                &lt;input class=&quot;form-control&quot; name=&quot;[1].Quantity&quot; min=&quot;1&quot; type=&quot;number&quot; value=&quot;1&quot; />
            &lt;/div>
        
    &lt;/div>

    &lt;div class=&quot;form-group row&quot;>
        
        
            
            &lt;div class=&quot;col-md-7&quot;>
                &lt;input class=&quot;form-control&quot; name=&quot;[2].Sku&quot; type=&quot;text&quot; value=&quot;&quot; />
            &lt;/div>
            &lt;div class=&quot;col-md-5&quot;>
                &lt;input class=&quot;form-control&quot; name=&quot;[2].Quantity&quot; min=&quot;1&quot; type=&quot;number&quot; value=&quot;1&quot; />
            &lt;/div>
        
    &lt;/div>

    &lt;div class=&quot;form-group row&quot;>
        
        
            
            &lt;div class=&quot;col-md-7&quot;>
                &lt;input class=&quot;form-control&quot; name=&quot;[3].Sku&quot; type=&quot;text&quot; value=&quot;&quot; />
            &lt;/div>
            &lt;div class=&quot;col-md-5&quot;>
                &lt;input class=&quot;form-control&quot; name=&quot;[3].Quantity&quot; min=&quot;1&quot; type=&quot;number&quot; value=&quot;1&quot; />
            &lt;/div>
        
    &lt;/div>

    &lt;div class=&quot;form-group row&quot;>
        
        
            
            &lt;div class=&quot;col-md-7&quot;>
                &lt;input class=&quot;form-control&quot; name=&quot;[4].Sku&quot; type=&quot;text&quot; value=&quot;&quot; />
            &lt;/div>
            &lt;div class=&quot;col-md-5&quot;>
                &lt;input class=&quot;form-control&quot; name=&quot;[4].Quantity&quot; min=&quot;1&quot; type=&quot;number&quot; value=&quot;1&quot; />
            &lt;/div>
        
    &lt;/div>

    &lt;div class=&quot;form-group row&quot;>
        
        
            
            &lt;div class=&quot;col-md-7&quot;>
                &lt;input class=&quot;form-control&quot; name=&quot;[5].Sku&quot; type=&quot;text&quot; value=&quot;&quot; />
            &lt;/div>
            &lt;div class=&quot;col-md-5&quot;>
                &lt;input class=&quot;form-control&quot; name=&quot;[5].Quantity&quot; min=&quot;1&quot; type=&quot;number&quot; value=&quot;1&quot; />
            &lt;/div>
        
    &lt;/div>


                    &lt;p>&lt;a class=&quot;small&quot; href=&quot;/B2B-store/bulkorder&quot;>Show More Entry Fields&lt;/a>&lt;/p>
                &lt;button class=&quot;btn btn-primary&quot; type=&quot;submit&quot;>Add to cart&lt;/button>
                &lt;/div>
            &lt;/form>
        &lt;dl ng-show=&quot;state == 'csv'&quot;>
            &lt;dt>Want to save time and order faster?&lt;/dt>
            &lt;dd>Simply copy and paste item records from your CSV file into the field below using the following format: &lt;strong>SKU,Quantity&lt;/strong>. Each record should starts from a new line.&lt;/dd>
        &lt;/dl>
        &lt;form action=&quot;/B2B-store/bulkorder/addcsvitems&quot; method=&quot;POST&quot; ng-show=&quot;state == 'csv'&quot;>
            &lt;div class=&quot;form-group&quot;>
                &lt;textarea name=&quot;csv&quot; placeholder=&quot;Formatting example:
6AF8SM0VPFV6,10
3RO1GEQI34E8,20&quot; rows=&quot;10&quot; class=&quot;form-control&quot;>&lt;/textarea>
            &lt;/div>
            &lt;button class=&quot;btn btn-primary&quot; type=&quot;submit&quot;>Add to cart&lt;/button>
        &lt;/form>
    &lt;/div>
&lt;/div>

    


            
            
    
         Cart 
    


        
        
    
        
        
        
            Search
        
    
            Searching...
            No products found
        
    
        &lt;a ng-href=&quot;{{ match.model.url }}&quot;
           tabindex=&quot;-1&quot;
           ng-bind-html=&quot;match.label | uibTypeaheadHighlight:query&quot;
           ng-attr-title=&quot;&quot;>&lt;/a>
    
    
        &lt;ul class=&quot;dropdown-menu&quot; ng-show=&quot;isOpen() &amp;&amp; !moveInProgress&quot; ng-style=&quot;{top: position().top+'px', left: position().left+'px'}&quot; role=&quot;listbox&quot; aria-hidden=&quot;{{!isOpen()}}&quot;>
            &lt;li class=&quot;dropdown-header&quot; ng-repeat-start=&quot;(group, groupMatches) in matches | groupBy: 'model.within'&quot; ng-bind=&quot;$parent.$parent.$ctrl[group + 'Label']&quot;>&lt;/li>
            &lt;li class=&quot;uib-typeahead-match&quot; ng-repeat=&quot;match in groupMatches track by $index&quot; ng-repeat-end ng-class=&quot;{active: isActive(match.model.index) }&quot; ng-mouseenter=&quot;selectActive(match.model.index)&quot; ng-click=&quot;selectMatch(match.model.index, $event)&quot; role=&quot;option&quot; id=&quot;{{::match.id}}&quot;>
                &lt;div uib-typeahead-match index=&quot;match.model.index&quot; match=&quot;match&quot; query=&quot;query&quot; template-url=&quot;templateUrl&quot;>&lt;/div>
            &lt;/li>
        &lt;/ul>
    



    </value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[@class=&quot;js&quot;]/body[@class=&quot;customer-logged-in template-search ng-scope&quot;]/header[@class=&quot;hidden-print hidden-xs hidden-sm&quot;]/nav[@class=&quot;navbar navbar-inverse-light navbar-static-top&quot;]/div[@class=&quot;container&quot;]</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Contact Us'])[1]/following::div[1]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Find a Branch'])[1]/following::div[1]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//nav[3]/div</value>
   </webElementXpaths>
</WebElementEntity>
