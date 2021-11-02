import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('https://webstore-storefront.qa.govirto.com/B2B-store/#/')

WebUI.setText(findTestObject('Page_Virto Commerce Demo B2B Online Shoppin_3adbc8/input_Sign in_customeruser_name'), 'userQA')

WebUI.setEncryptedText(findTestObject('Page_Virto Commerce Demo B2B Online Shoppin_3adbc8/input_Required_customerpassword'), 
    'Z7r/5vJ5kyd94QMP3YfOwA==')

WebUI.click(findTestObject('Page_Virto Commerce Demo B2B Online Shoppin_3adbc8/input_Forgot your password_btn btn-primary _744e93'))

WebUI.setText(findTestObject('Page_Virto Commerce Demo B2B My Account - B_4b6452/input_All products_q'), 'laser')

WebUI.click(findTestObject('Page_Virto Commerce Demo B2B My Account - B_4b6452/button_Search'))

WebUI.click(findTestObject('Myobjects/Search/h2_Your search for laser revealed the following'))

WebUI.click(findTestObject('Myobjects/Search/div_All products                           _87dcd7'))

WebUI.setText(findTestObject('Myobjects/Bulk Order/input_Cart_q'), 
    'pri')

WebUI.click(findTestObject('Myobjects/Search/button_Search'))

WebUI.click(findTestObject('Myobjects/Search/h2_Your search for pri revealed the following'))

WebUI.click(findTestObject('Object Repository/Myobjects/Search/Page_Virto Commerce Demo B2B Online Shoppin_3adbc8/a_Bulk Order Pad'))

WebUI.setText(findTestObject('Myobjects/Bulk Order/input_Quantity_0.Sku'), 
    '22A447')

WebUI.setText(findTestObject('Myobjects/Bulk Order/input_Quantity_1.Sku'), 
    '566903892')

WebUI.click(findTestObject('Object Repository/Myobjects/Search/Page_Virto Commerce Demo B2B Online Shoppin_3adbc8/button_Add to cart'))

WebUI.click(findTestObject('Object Repository/Myobjects/Search/Page_Virto Commerce Demo B2B Online Shoppin_3adbc8/a_Bulk Order Pad'))

WebUI.setText(findTestObject('Myobjects/Bulk Order/input_Quantity_0.Sku'), 
    '566903892')

WebUI.setText(findTestObject('Myobjects/Bulk Order/input_Quantity_1.Sku'), 
    '343212sx')

WebUI.click(findTestObject('Object Repository/Myobjects/Search/Page_Virto Commerce Demo B2B Online Shoppin_3adbc8/button_Add to cart'))

WebUI.click(findTestObject('Myobjects/Search/dl_Products with following SKUs was not add_c30de0'))

