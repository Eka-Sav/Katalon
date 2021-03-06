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
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable

//def CheckAddressResult = WebUI.verifyElementPresent(findTestObject('Object Repository/CartCheckoutPage/Page_Electronics - Checkout/ALogin'), 5, FailureHandling.OPTIONAL)
def CheckAddressResult = WebUI.verifyElementPresent(findTestObject('UI-Electronics/CartCheckoutPage/Page_Electronics - Checkout/SelectAddress'), 5, FailureHandling.OPTIONAL)

if (CheckAddressResult == false) {
	
	WebUI.click(findTestObject('UI-Electronics/CartCheckoutPage/Page_Electronics - Checkout/InputEmailCheckout'))
	
		WebUI.setText(findTestObject('UI-Electronics/CartCheckoutPage/Page_Electronics - Checkout/InputEmailCheckout'), GlobalVariable.contactEmail)
	
		WebUI.click(findTestObject('UI-Electronics/CartCheckoutPage/Page_Electronics - Checkout/FirstNameCheckout'))
	
		WebUI.setText(findTestObject('UI-Electronics/CartCheckoutPage/Page_Electronics - Checkout/FirstNameCheckout'), GlobalVariable.contactFirstName)
	
		WebUI.click(findTestObject('UI-Electronics/CartCheckoutPage/Page_Electronics - Checkout/LastNameCheckout'))
	
		WebUI.setText(findTestObject('UI-Electronics/CartCheckoutPage/Page_Electronics - Checkout/LastNameCheckout'), GlobalVariable.contactLastName)
	
		WebUI.click(findTestObject('UI-Electronics/CartCheckoutPage/Page_Electronics - Checkout/CompanyCheckout'))
	
		WebUI.setText(findTestObject('UI-Electronics/CartCheckoutPage/Page_Electronics - Checkout/CompanyCheckout'), GlobalVariable.company)
	
		WebUI.click(findTestObject('UI-Electronics/CartCheckoutPage/Page_Electronics - Checkout/AddressCheckout'))
	
		WebUI.setText(findTestObject('UI-Electronics/CartCheckoutPage/Page_Electronics - Checkout/AddressCheckout'), GlobalVariable.organizationAddress)
	
		WebUI.click(findTestObject('UI-Electronics/CartCheckoutPage/Page_Electronics - Checkout/AptCheckout'))
	
		WebUI.setText(findTestObject('UI-Electronics/CartCheckoutPage/Page_Electronics - Checkout/AptCheckout'), GlobalVariable.organizationApt)
	
		WebUI.click(findTestObject('UI-Electronics/CartCheckoutPage/Page_Electronics - Checkout/CityCheckout'))
	
		WebUI.setText(findTestObject('UI-Electronics/CartCheckoutPage/Page_Electronics - Checkout/CityCheckout'), GlobalVariable.organizationCity)
	
		WebUI.selectOptionByIndex(findTestObject('UI-Electronics/CartCheckoutPage/Page_Electronics - Checkout/SelectCountry'), 2)
	
		WebUI.selectOptionByIndex(findTestObject('UI-Electronics/CartCheckoutPage/Page_Electronics - Checkout/SelectRegion'), 2)
	
		WebUI.click(findTestObject('UI-Electronics/CartCheckoutPage/Page_Electronics - Checkout/ZIpCheckout'))
	
		WebUI.setText(findTestObject('UI-Electronics/CartCheckoutPage/Page_Electronics - Checkout/ZIpCheckout'), GlobalVariable.organizationZip)
	
		} else {
	
		WebUI.selectOptionByIndex(findTestObject('UI-Electronics/CartCheckoutPage/Page_Electronics - Checkout/SelectAddress'), 1)

}
		//GlobalVariable.priceFromPopup = WebUI.getText(findTestObject('UI-Electronics/CartCheckoutPage/Page_Electronics - Checkout/DivSubtotalInCart'))
		//println ("PRICE FROM POPUP IS : "+GlobalVariable.priceFromPopup)
		GlobalVariable.itemsTotalFromCart = WebUI.getText(findTestObject('UI-Electronics/Page_Electronics - Checkout/SpanTotal'))
		def TotalWithoutDollar = GlobalVariable.itemsTotalFromCart;
		TotalWithoutDollar = TotalWithoutDollar.substring(1);
		//GlobalVariable.itemsTotalFromCart = Float.parseFloat(GlobalVariable.itemsTotalFromCart)
		println ("TOTAL PRICE IS : "+GlobalVariable.itemsTotalFromCart)
		WebUI.verifyEqual(TotalWithoutDollar, "89.99")
