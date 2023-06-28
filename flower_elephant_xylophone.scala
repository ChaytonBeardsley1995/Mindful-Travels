import scala.collection.mutable.ListBuffer

// Defining class MindfulTravels

class MindfulTravels {
  
  // Creating private instance variables
  private var totalDistance: Double = 0.0
  private var placesVisited: ListBuffer[String] = ListBuffer[String]()
  
  // Adding getter and setter methods
  
  // Get total distance method
  def getTotalDistance = totalDistance
  // Set total distance method
  def setTotalDistance(newDistance: Double) = totalDistance = newDistance
  // Get places visited method 
  def getPlacesVisited = placesVisited
  // Set places visited method 
  def setPlacesVisited(newPlacesVisited: ListBuffer[String]) = placesVisited = newPlacesVisited
  
  // Defining a method to add a new place
  def addPlace(placeName: String, distanceTraveled: Double) = {
    
    // Adding the place to the list of places visited
    placesVisited += placeName
    
    // Adding the distance traveled to the total distance
    totalDistance += distanceTraveled
  }
  
  // Defining a method to print out places visited
  def printPlacesVisited() : Unit = {
    for (place <- placesVisited) {
      println(place)
    }
  }
  
}

// Creating instance of MindfulTravels

val mindfulTravels = new MindfulTravels()

// Adding various places

mindfulTravels.addPlace("Los Angeles", 50.0)
mindfulTravels.addPlace("San Francisco", 75.0)
mindfulTravels.addPlace("Las Vegas", 100.0)

// Printing places visited

mindfulTravels.printPlacesVisited()

// Output
// Los Angeles
// San Francisco
// Las Vegas